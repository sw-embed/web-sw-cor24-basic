pub struct Demo {
    pub name: &'static str,
    pub source: &'static str,
    pub interactive: bool,
}

pub static DEMOS: &[Demo] = &[
    Demo {
        name: "calc",
        source: include_str!("../examples/calc.bas"),
        interactive: false,
    },
    Demo {
        name: "count",
        source: include_str!("../examples/count.bas"),
        interactive: false,
    },
    Demo {
        name: "factorial",
        source: include_str!("../examples/factorial.bas"),
        interactive: false,
    },
    Demo {
        name: "fibonacci",
        source: include_str!("../examples/fibonacci.bas"),
        interactive: false,
    },
    Demo {
        name: "fizzbuzz",
        source: include_str!("../examples/fizzbuzz.bas"),
        interactive: false,
    },
    Demo {
        name: "guess",
        source: include_str!("../examples/guess.bas"),
        interactive: true,
    },
    Demo {
        name: "hello",
        source: include_str!("../examples/hello.bas"),
        interactive: false,
    },
    Demo {
        name: "memdump",
        source: include_str!("../examples/memdump.bas"),
        interactive: false,
    },
    Demo {
        name: "robot-chase",
        source: include_str!("../examples/robot-chase.bas"),
        interactive: true,
    },
    Demo {
        name: "startrek",
        source: include_str!("../examples/startrek.bas"),
        interactive: true,
    },
    Demo {
        name: "trek-adventure",
        source: include_str!("../examples/trek-adventure.bas"),
        interactive: true,
    },
];

pub fn default_demo_index() -> usize {
    DEMOS.iter().position(|d| d.name == "hello").unwrap_or(0)
}
