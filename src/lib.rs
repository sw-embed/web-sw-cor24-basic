use gloo::timers::callback::Timeout;
use web_sys::{
    Element, HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement,
    KeyboardEvent,
};
use yew::prelude::*;

pub mod config;
pub mod demos;
pub mod runner;

use demos::{DEMOS, default_demo_index};
use runner::Session;

const DEFAULT_MAX_INSTRS: u64 = 200_000_000;
const TICK_DELAY_MS: u32 = 0;

fn now_ms() -> f64 {
    js_sys::Date::now()
}

fn defer_focus_input(node: NodeRef) {
    Timeout::new(0, move || {
        if let Some(el) = node.cast::<HtmlInputElement>() {
            let _ = el.focus();
        }
    })
    .forget();
}

fn defer_focus_select(node: NodeRef) {
    Timeout::new(0, move || {
        if let Some(el) = node.cast::<HtmlSelectElement>() {
            let _ = el.focus();
        }
    })
    .forget();
}

fn defer_focus_button(node: NodeRef) {
    Timeout::new(0, move || {
        if let Some(el) = node.cast::<HtmlButtonElement>() {
            let _ = el.focus();
        }
    })
    .forget();
}

#[derive(PartialEq, Clone, Copy)]
pub enum HelpTab {
    Guide,
    Reference,
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
    InputChanged(String),
    InputSubmit,
    ShowHelp,
    HideHelp,
    SelectHelpTab(HelpTab),
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
    input_line: String,
    awaiting_input: bool,
    output_ref: NodeRef,
    input_ref: NodeRef,
    select_ref: NodeRef,
    run_ref: NodeRef,
    show_help: bool,
    help_tab: HelpTab,
    focus_input_pending: bool,
    focus_run_pending: bool,
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
            self.input_line.clear();
            self.awaiting_input = false;
        }
    }

    fn start_run(&mut self, ctx: &Context<Self>) {
        let interactive = DEMOS
            .get(self.selected)
            .map(|d| d.interactive)
            .unwrap_or(false);
        self.session = Some(if interactive {
            Session::new_interactive(&self.source)
        } else {
            Session::new(&self.source)
        });
        self.input_line.clear();
        self.awaiting_input = false;
        self.running = true;
        self.error = false;
        self.budget_exhausted = false;
        self.output.clear();
        self.started_at = now_ms();
        self.elapsed_ms = 0.0;
        self.status = "running...".into();
        self.focus_input_pending = interactive;
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
            input_line: String::new(),
            awaiting_input: false,
            output_ref: NodeRef::default(),
            input_ref: NodeRef::default(),
            select_ref: NodeRef::default(),
            run_ref: NodeRef::default(),
            show_help: false,
            help_tab: HelpTab::Guide,
            focus_input_pending: false,
            focus_run_pending: false,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if let Some(el) = self.output_ref.cast::<Element>() {
            el.set_scroll_top(el.scroll_height());
        }

        if first_render {
            defer_focus_select(self.select_ref.clone());
        }
        if self.focus_run_pending {
            defer_focus_button(self.run_ref.clone());
            self.focus_run_pending = false;
        }
        if self.awaiting_input || self.focus_input_pending {
            defer_focus_input(self.input_ref.clone());
            self.focus_input_pending = false;
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectDemo(i) => {
                self.load_demo(i);
                self.max_instrs = DEFAULT_MAX_INSTRS;
                self.focus_run_pending = true;
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
                let interactive = DEMOS
                    .get(self.selected)
                    .map(|d| d.interactive)
                    .unwrap_or(false);
                if !interactive {
                    let remaining = self.max_instrs.saturating_sub(session.instructions());
                    if remaining == 0 {
                        self.budget_exhausted = true;
                        let instrs = session.instructions();
                        self.finish(format!("halted (budget) -- {} instrs", instrs), true);
                        return true;
                    }
                }
                let result = session.tick();
                if session.is_awaiting_input() {
                    self.awaiting_input = true;
                    self.output = session.output();
                    self.elapsed_ms = now_ms() - self.started_at;
                    self.status = format!(
                        "awaiting input ({} instrs, {:.0} ms)",
                        session.instructions(),
                        self.elapsed_ms
                    );
                    return true;
                }
                if result.done {
                    let instrs = session.instructions();
                    let reason = session.stop_reason();
                    let halted = session.is_halted();
                    self.finish(
                        format!(
                            "{} ({} instrs, {:.0} ms)",
                            reason,
                            instrs,
                            now_ms() - self.started_at
                        ),
                        !halted,
                    );
                } else {
                    self.output = session.output();
                    self.elapsed_ms = now_ms() - self.started_at;
                    self.status = format!(
                        "running... {} instrs, {:.0} ms",
                        session.instructions(),
                        self.elapsed_ms
                    );
                    self.schedule_tick(ctx);
                }
                true
            }
            Msg::InputChanged(v) => {
                self.input_line = v;
                false
            }
            Msg::InputSubmit => {
                if !self.running {
                    return false;
                }
                let line = std::mem::take(&mut self.input_line);
                let was_awaiting = self.awaiting_input;
                if let Some(session) = self.session.as_mut() {
                    if was_awaiting {
                        // TTY-style echo: the user's typed line should appear
                        // in the output panel. No trailing newline so the
                        // program's response can stick to the same line as
                        // the INPUT prompt.
                        session.echo_input(&line);
                    }
                    session.feed_input(&line);
                }
                if was_awaiting {
                    self.output = self
                        .session
                        .as_ref()
                        .map(|s| s.output())
                        .unwrap_or_default();
                    self.awaiting_input = false;
                    self.status = "running...".into();
                    self.schedule_tick(ctx);
                }
                true
            }
            Msg::KeyDown(e) => {
                if e.key() == "Enter" && (e.ctrl_key() || e.meta_key()) {
                    e.prevent_default();
                    ctx.link().send_message(Msg::Run);
                } else if e.key() == "Escape" {
                    if self.show_help {
                        e.prevent_default();
                        ctx.link().send_message(Msg::HideHelp);
                    } else if self.running {
                        e.prevent_default();
                        ctx.link().send_message(Msg::Stop);
                    }
                }
                false
            }
            Msg::ShowHelp => {
                self.show_help = true;
                true
            }
            Msg::HideHelp => {
                self.show_help = false;
                true
            }
            Msg::SelectHelpTab(t) => {
                self.help_tab = t;
                true
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
        let on_input_change = ctx.link().callback(|e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            Msg::InputChanged(target.value())
        });
        let on_input_submit = ctx.link().callback(|_| Msg::InputSubmit);
        let on_input_keydown = ctx.link().callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                Msg::InputSubmit
            } else {
                Msg::KeyDown(e)
            }
        });
        let on_help_open = ctx.link().callback(|_| Msg::ShowHelp);
        let on_help_close = ctx.link().callback(|_| Msg::HideHelp);
        let on_help_guide = ctx.link().callback(|_| Msg::SelectHelpTab(HelpTab::Guide));
        let on_help_ref = ctx.link().callback(|_| Msg::SelectHelpTab(HelpTab::Reference));
        let interactive = DEMOS
            .get(self.selected)
            .map(|d| d.interactive)
            .unwrap_or(false);
        let show_input_row = self.running && interactive;

        let status_class = if self.error {
            "status status-error"
        } else {
            "status"
        };
        let run_button = if self.running {
            html! { <button ref={self.run_ref.clone()} onclick={on_stop}>{ "Stop" }</button> }
        } else {
            html! { <button ref={self.run_ref.clone()} onclick={on_run}>{ "Run" }</button> }
        };

        let help_dialog = if self.show_help {
            let stop = Callback::from(|e: MouseEvent| e.stop_propagation());
            let guide_class = if self.help_tab == HelpTab::Guide { "help-tab help-tab-active" } else { "help-tab" };
            let ref_class = if self.help_tab == HelpTab::Reference { "help-tab help-tab-active" } else { "help-tab" };
            let body = match self.help_tab {
                HelpTab::Guide => help_guide_html(),
                HelpTab::Reference => help_reference_html(),
            };
            html! {
                <div class="help-overlay" onclick={on_help_close.clone()}>
                    <div class="help-dialog" onclick={stop}>
                        <div class="help-header">
                            <h2>{ "BASIC Help" }</h2>
                            <button class="help-close" aria-label="Close" onclick={on_help_close.clone()}>
                                { "\u{2715}" }
                            </button>
                        </div>
                        <div class="help-tabs" role="tablist">
                            <button class={guide_class} onclick={on_help_guide}>{ "User Guide" }</button>
                            <button class={ref_class} onclick={on_help_ref}>{ "Reference" }</button>
                        </div>
                        <div class="help-content">{ body }</div>
                    </div>
                </div>
            }
        } else {
            html! {}
        };

        html! {
            <>
            { help_dialog }
            <a href="https://github.com/sw-embed/web-sw-cor24-basic" class="github-corner"
               aria-label="View source on GitHub" target="_blank">
                <svg width="80" height="80" viewBox="0 0 250 250" aria-hidden="true">
                    <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z" />
                    <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 \
                        120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 \
                        C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor"
                        style="transform-origin:130px 106px;" class="octo-arm" />
                    <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 \
                        139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 \
                        159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 \
                        C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 \
                        216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 \
                        198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 \
                        152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z"
                        fill="currentColor" />
                </svg>
            </a>
            <main class="page" onkeydown={on_keydown.clone()}>
                <header class="chrome">
                    <h1>{ "web-sw-cor24-basic" }</h1>
                    <div class="controls">
                        <select ref={self.select_ref.clone()} onchange={on_demo} disabled={self.running}>
                            { for DEMOS.iter().enumerate().map(|(i, d)| html! {
                                <option value={i.to_string()} selected={i == self.selected}>
                                    { d.name }
                                </option>
                            })}
                        </select>
                        { run_button }
                        <button class="secondary" onclick={on_reset} disabled={self.running}>{ "Reset" }</button>
                        <button class="secondary" onclick={on_clear}>{ "Clear" }</button>
                        <button class="secondary" onclick={on_help_open}>{ "Help" }</button>
                    </div>
                </header>
                <div class="workspace">
                <section class="panel panel-src">
                    <label>{ "source (.bas)" }</label>
                    <textarea
                        class="src"
                        spellcheck="false"
                        value={self.source.clone()}
                        oninput={on_src}
                        onkeydown={on_keydown.clone()}
                    />
                </section>
                <section class="panel panel-out">
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
                    <pre class="out" ref={self.output_ref.clone()}>{ &self.output }</pre>
                    { if show_input_row {
                        html! {
                            <div class="input-row">
                                <label>{ "input:" }</label>
                                <input
                                    ref={self.input_ref.clone()}
                                    type="text"
                                    value={self.input_line.clone()}
                                    oninput={on_input_change}
                                    onkeydown={on_input_keydown}
                                    autofocus=true
                                />
                                <button onclick={on_input_submit}>{ "Send" }</button>
                            </div>
                        }
                    } else { html! {} }}
                </section>
                </div>
            </main>
            <footer>
                <span>{"MIT License"}</span>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <span>{"\u{00a9} 2026 Michael A Wright"}</span>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://makerlisp.com" target="_blank">{"COR24-TB"}</a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://software-wrighter-lab.github.io/" target="_blank">{"Blog"}</a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://discord.com/invite/Ctzk5uHggZ" target="_blank">{"Discord"}</a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://www.youtube.com/@SoftwareWrighter" target="_blank">{"YouTube"}</a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://github.com/sw-embed/web-sw-cor24-basic/blob/main/docs/demos.md" target="_blank">{"Demo Documentation"}</a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <a href="https://github.com/sw-embed/web-sw-cor24-basic/blob/main/CHANGES.md" target="_blank">{"Changes"}</a>
                <span class="footer-sep">{"\u{00b7}"}</span>
                <span>{ format!("{} \u{00b7} {} \u{00b7} {}",
                    env!("BUILD_HOST"),
                    env!("BUILD_SHA"),
                    env!("BUILD_TIMESTAMP"),
                ) }</span>
            </footer>
            </>
        }
    }
}

fn help_guide_html() -> Html {
    html! {
        <>
        <h3>{ "Getting started" }</h3>
        <p>{ "COR24 BASIC v1 is a line-numbered BASIC. Pick a demo from the dropdown, hit " }<b>{ "Run" }</b>{ ", or edit the source and press " }<kbd>{ "Cmd/Ctrl+Enter" }</kbd>{ ". " }<kbd>{ "Esc" }</kbd>{ " stops a running program." }</p>
        <p>{ "Programs end with " }<code>{ "BYE" }</code>{ " (halt) or " }<code>{ "END" }</code>{ ". Use " }<code>{ "STOP" }</code>{ " to pause; " }<code>{ "CONT" }</code>{ " in immediate mode resumes." }</p>

        <h3>{ "Variables and types" }</h3>
        <p>{ "26 single-letter variables A..Z, all 24-bit signed integers (\u{2248} \u{00b1}8.3M). No floating point. No string variables (no A$). String " }<i>{ "literals" }</i>{ " work in PRINT only." }</p>

        <h3>{ "PRINT and INPUT" }</h3>
<pre>{ "10 PRINT \"HELLO\", X    REM comma = next 14-col tab stop\n20 PRINT \"X=\"; X        REM semicolon = no newline\n30 INPUT \"GUESS \"; G    REM read integer into G" }</pre>

        <h3>{ "Control flow" }</h3>
<pre>{ "100 IF X < 10 THEN PRINT \"SMALL\"\n110 IF X = 0 THEN GOTO 200\n200 FOR I = 1 TO 10 STEP 2\n210   PRINT I\n220 NEXT I\n300 ON K GOTO 310, 320, 330\n400 GOSUB 500\n500 RETURN" }</pre>

        <h3>{ "Arrays (DIM)" }</h3>
<pre>{ "10 DIM A(99)            REM A(0)..A(99), shared 1024-int pool\n20 LET A(5) = 42" }</pre>

        <h3>{ "DATA / READ / RESTORE" }</h3>
<pre>{ "10 DATA 3, 1, 4, 1, 5, 9\n20 FOR I=1 TO 6\n30 READ X : PRINT X\n40 NEXT I\n50 RESTORE          REM rewind data pointer" }</pre>

        <h3>{ "Random numbers from a seed" }</h3>
        <p>{ "There is " }<b>{ "no built-in RND or RANDOMIZE" }</b>{ " in this dialect. Roll your own LCG from a seed. The " }<code>{ "robot-chase" }</code>{ " demo uses:" }</p>
<pre>{ "10 LET R = 42                        REM seed\n20 LET R = (R*97 + 1) MOD 8191        REM step\n30 LET T = (R MOD 100) + 1            REM 1..100" }</pre>
        <p>{ "Call the step every time you need a new draw. Same seed \u{2192} same sequence (deterministic, good for testing)." }</p>

        <h3>{ "Memory: PEEK / POKE" }</h3>
        <p>{ "1024 bytes of user memory at addresses 0..1023. " }<code>{ "POKE addr, val" }</code>{ " writes; " }<code>{ "PEEK(addr)" }</code>{ " reads." }</p>

        <h3>{ "Tips" }</h3>
        <ul>
            <li>{ "Line numbers must be ascending in storage; gaps are fine." }</li>
            <li>{ "Multiple statements per line with " }<code>{ ":" }</code>{ "." }</li>
            <li>{ "If a program runs out of instruction budget, click " }<i>{ "Increase budget 4x" }</i>{ " in the status bar." }</li>
        </ul>
        </>
    }
}

fn help_reference_html() -> Html {
    html! {
        <>
        <h3>{ "Statements" }</h3>
        <table class="help-table">
            <tr><td><code>{ "LET v = expr" }</code></td><td>{ "assignment (LET optional)" }</td></tr>
            <tr><td><code>{ "PRINT expr [,;] ..." }</code></td><td>{ ", = tab; ; = no newline" }</td></tr>
            <tr><td><code>{ "INPUT [\"prompt\";] v" }</code></td><td>{ "read integer" }</td></tr>
            <tr><td><code>{ "IF cond THEN stmt" }</code></td><td>{ "single-line conditional" }</td></tr>
            <tr><td><code>{ "FOR v=a TO b [STEP s]" }</code></td><td>{ "loop (max 16 nested)" }</td></tr>
            <tr><td><code>{ "NEXT v" }</code></td></tr>
            <tr><td><code>{ "GOTO n" }</code></td></tr>
            <tr><td><code>{ "GOSUB n / RETURN" }</code></td><td>{ "max 64-deep call stack" }</td></tr>
            <tr><td><code>{ "ON e GOTO|GOSUB n1, n2, ..." }</code></td><td>{ "1-based dispatch" }</td></tr>
            <tr><td><code>{ "DIM a(n)" }</code></td><td>{ "array, shared 1024-int pool" }</td></tr>
            <tr><td><code>{ "DATA v1, v2, ..." }</code></td></tr>
            <tr><td><code>{ "READ v" }</code></td></tr>
            <tr><td><code>{ "RESTORE" }</code></td><td>{ "rewind DATA pointer" }</td></tr>
            <tr><td><code>{ "POKE addr, val" }</code></td></tr>
            <tr><td><code>{ "REM ..." }</code></td><td>{ "comment to end of line" }</td></tr>
            <tr><td><code>{ "STOP / CONT" }</code></td><td>{ "pause / resume" }</td></tr>
            <tr><td><code>{ "END / BYE" }</code></td><td>{ "halt program" }</td></tr>
        </table>

        <h3>{ "Operators (low to high precedence)" }</h3>
        <table class="help-table">
            <tr><td>{ "logical" }</td><td><code>{ "AND  OR" }</code></td></tr>
            <tr><td>{ "bitwise" }</td><td><code>{ "BAND  BOR  BXOR  SHL  SHR" }</code></td></tr>
            <tr><td>{ "relational" }</td><td><code>{ "=  <>  <  <=  >  >=" }</code></td></tr>
            <tr><td>{ "additive" }</td><td><code>{ "+  -" }</code></td></tr>
            <tr><td>{ "multiplicative" }</td><td><code>{ "*  /  MOD" }</code></td></tr>
            <tr><td>{ "unary" }</td><td><code>{ "+  -" }</code></td></tr>
        </table>
        <p><i>{ "No " }</i><code>{ "^" }</code><i>{ " (exponent), no " }</i><code>{ "NOT" }</code><i>{ "." }</i></p>

        <h3>{ "Built-in functions" }</h3>
        <table class="help-table">
            <tr><td><code>{ "ABS(x)" }</code></td><td>{ "absolute value" }</td></tr>
            <tr><td><code>{ "CHR$(n)" }</code></td><td>{ "char with ASCII code n (PRINT only)" }</td></tr>
            <tr><td><code>{ "PEEK(addr)" }</code></td><td>{ "byte at addr (0..1023 = user memory)" }</td></tr>
        </table>
        <p><i>{ "No RND, RANDOMIZE, INT, SGN, SQR, MID$, LEFT$, RIGHT$, STR$, VAL, SIN, COS, DEF FN." }</i></p>

        <h3>{ "Limits" }</h3>
        <table class="help-table">
            <tr><td>{ "variables" }</td><td>{ "26 (A..Z), 24-bit signed" }</td></tr>
            <tr><td>{ "line numbers" }</td><td>{ "0..65535" }</td></tr>
            <tr><td>{ "program size" }</td><td>{ "16384 bytes tokenised" }</td></tr>
            <tr><td>{ "array pool" }</td><td>{ "1024 ints across all DIMs" }</td></tr>
            <tr><td>{ "GOSUB stack" }</td><td>{ "64 deep" }</td></tr>
            <tr><td>{ "FOR stack" }</td><td>{ "16 deep" }</td></tr>
            <tr><td>{ "user memory" }</td><td>{ "1024 bytes (PEEK/POKE 0..1023)" }</td></tr>
        </table>

        <h3>{ "Runtime errors" }</h3>
        <table class="help-table">
            <tr><td>{ "1" }</td><td>{ "syntax" }</td></tr>
            <tr><td>{ "2" }</td><td>{ "unknown statement" }</td></tr>
            <tr><td>{ "3" }</td><td>{ "line not found" }</td></tr>
            <tr><td>{ "4" }</td><td>{ "DIM pool exhausted" }</td></tr>
            <tr><td>{ "5" }</td><td>{ "divide by zero" }</td></tr>
            <tr><td>{ "6" }</td><td>{ "GOSUB stack overflow" }</td></tr>
            <tr><td>{ "7" }</td><td>{ "RETURN without GOSUB" }</td></tr>
            <tr><td>{ "8" }</td><td>{ "array bounds" }</td></tr>
            <tr><td>{ "9" }</td><td>{ "FOR stack overflow" }</td></tr>
            <tr><td>{ "13" }</td><td>{ "READ past end of DATA" }</td></tr>
            <tr><td>{ "16" }</td><td>{ "CONT without STOP" }</td></tr>
        </table>
        </>
    }
}
