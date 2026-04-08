use gloo::timers::callback::Timeout;
use web_sys::{HtmlSelectElement, HtmlTextAreaElement, KeyboardEvent};
use yew::prelude::*;

pub mod config;
pub mod demos;
pub mod runner;

use demos::{default_demo_index, DEMOS};
use runner::Session;

const DEFAULT_MAX_INSTRS: u64 = 200_000_000;
const TICK_DELAY_MS: u32 = 0;

fn now_ms() -> f64 {
    js_sys::Date::now()
}

pub enum Msg {
    SelectDemo(usize),
    SourceChanged(String),
    Run,
    Tick,
    Stop,
    Reset,
    Clear,
    IncreaseBudget,
    KeyDown(KeyboardEvent),
}

pub struct App {
    selected: usize,
    source: String,
    output: String,
    status: String,
    error: bool,
    session: Option<Session>,
    running: bool,
    max_instrs: u64,
    started_at: f64,
    elapsed_ms: f64,
    budget_exhausted: bool,
}

impl App {
    fn load_demo(&mut self, idx: usize) {
        if let Some(demo) = DEMOS.get(idx) {
            self.selected = idx;
            self.source = demo.source.to_string();
            self.output.clear();
            self.status = "idle".into();
            self.error = false;
            self.session = None;
            self.running = false;
            self.budget_exhausted = false;
            self.elapsed_ms = 0.0;
        }
    }

    fn start_run(&mut self, ctx: &Context<Self>) {
        self.session = Some(Session::new(&self.source));
        self.running = true;
        self.error = false;
        self.budget_exhausted = false;
        self.output.clear();
        self.started_at = now_ms();
        self.elapsed_ms = 0.0;
        self.status = "running...".into();
        self.schedule_tick(ctx);
    }

    fn schedule_tick(&self, ctx: &Context<Self>) {
        let link = ctx.link().clone();
        Timeout::new(TICK_DELAY_MS, move || link.send_message(Msg::Tick)).forget();
    }

    fn finish(&mut self, status: String, error: bool) {
        self.running = false;
        self.status = status;
        self.error = error;
        self.elapsed_ms = now_ms() - self.started_at;
        if let Some(s) = &self.session {
            let raw = s.output();
            self.output = raw;
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let idx = default_demo_index();
        let demo = &DEMOS[idx];
        Self {
            selected: idx,
            source: demo.source.to_string(),
            output: String::new(),
            status: "idle".into(),
            error: false,
            session: None,
            running: false,
            max_instrs: DEFAULT_MAX_INSTRS,
            started_at: 0.0,
            elapsed_ms: 0.0,
            budget_exhausted: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectDemo(i) => {
                self.load_demo(i);
                self.max_instrs = DEFAULT_MAX_INSTRS;
                true
            }
            Msg::SourceChanged(v) => {
                self.source = v;
                false
            }
            Msg::Run => {
                self.max_instrs = DEFAULT_MAX_INSTRS;
                self.start_run(ctx);
                true
            }
            Msg::IncreaseBudget => {
                self.max_instrs = self.max_instrs.saturating_mul(4);
                self.start_run(ctx);
                true
            }
            Msg::Stop => {
                if self.running {
                    self.finish("stopped".into(), false);
                }
                true
            }
            Msg::Reset => {
                let idx = self.selected;
                self.running = false;
                self.session = None;
                self.load_demo(idx);
                self.max_instrs = DEFAULT_MAX_INSTRS;
                true
            }
            Msg::Clear => {
                self.output.clear();
                if !self.running {
                    self.status = "idle".into();
                    self.error = false;
                    self.budget_exhausted = false;
                }
                true
            }
            Msg::Tick => {
                if !self.running {
                    return false;
                }
                let Some(session) = self.session.as_mut() else {
                    self.running = false;
                    return true;
                };
                let remaining = self.max_instrs.saturating_sub(session.instructions);
                if remaining == 0 {
                    self.budget_exhausted = true;
                    let instrs = session.instructions;
                    self.finish(format!("halted (budget) -- {} instrs", instrs), true);
                    return true;
                }
                let result = session.tick();
                if result.done {
                    let halted = session.halted;
                    let instrs = session.instructions;
                    let reason = session.stop_reason.clone();
                    self.finish(
                        if halted {
                            format!(
                                "done ({} instrs, {:.0} ms)",
                                instrs,
                                now_ms() - self.started_at
                            )
                        } else {
                            format!("{} ({} instrs)", reason, instrs)
                        },
                        !halted,
                    );
                } else {
                    self.output = session.output();
                    self.elapsed_ms = now_ms() - self.started_at;
                    self.status = format!(
                        "running... {} instrs, {:.0} ms",
                        session.instructions, self.elapsed_ms
                    );
                    self.schedule_tick(ctx);
                }
                true
            }
            Msg::KeyDown(e) => {
                if e.key() == "Enter" && (e.ctrl_key() || e.meta_key()) {
                    e.prevent_default();
                    ctx.link().send_message(Msg::Run);
                } else if e.key() == "Escape" && self.running {
                    e.prevent_default();
                    ctx.link().send_message(Msg::Stop);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_demo = ctx.link().callback(|e: Event| {
            let target: HtmlSelectElement = e.target_unchecked_into();
            let idx: usize = target.value().parse().unwrap_or(0);
            Msg::SelectDemo(idx)
        });
        let on_src = ctx.link().callback(|e: InputEvent| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            Msg::SourceChanged(target.value())
        });
        let on_run = ctx.link().callback(|_| Msg::Run);
        let on_stop = ctx.link().callback(|_| Msg::Stop);
        let on_reset = ctx.link().callback(|_| Msg::Reset);
        let on_clear = ctx.link().callback(|_| Msg::Clear);
        let on_inc = ctx.link().callback(|_| Msg::IncreaseBudget);
        let on_keydown = ctx.link().callback(Msg::KeyDown);

        let status_class = if self.error {
            "status status-error"
        } else {
            "status"
        };
        let run_button = if self.running {
            html! { <button onclick={on_stop}>{ "Stop" }</button> }
        } else {
            html! { <button onclick={on_run}>{ "Run" }</button> }
        };

        html! {
            <main class="page" onkeydown={on_keydown.clone()}>
                <header class="chrome">
                    <h1>{ "web-sw-cor24-basic" }</h1>
                    <div class="controls">
                        <select onchange={on_demo} disabled={self.running}>
                            { for DEMOS.iter().enumerate().map(|(i, d)| html! {
                                <option value={i.to_string()} selected={i == self.selected}>
                                    { d.name }
                                </option>
                            })}
                        </select>
                        { run_button }
                        <button class="secondary" onclick={on_reset} disabled={self.running}>{ "Reset" }</button>
                        <button class="secondary" onclick={on_clear}>{ "Clear" }</button>
                    </div>
                </header>
                <section class="panel">
                    <label>{ "source (.bas)" }</label>
                    <textarea
                        class="src"
                        rows="14"
                        spellcheck="false"
                        value={self.source.clone()}
                        oninput={on_src}
                        onkeydown={on_keydown.clone()}
                    />
                </section>
                <section class="panel">
                    <div class={status_class}>
                        { format!("status: {}", self.status) }
                        { if self.budget_exhausted {
                            html! {
                                <>
                                    { " -- " }
                                    <button class="link-btn" onclick={on_inc}>
                                        { "Increase budget 4x" }
                                    </button>
                                </>
                            }
                        } else { html! {} }}
                    </div>
                    <pre class="out">{ &self.output }</pre>
                </section>
            </main>
        }
    }
}
