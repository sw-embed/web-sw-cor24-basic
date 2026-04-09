pub struct Demo {
    pub name: &'static str,
    pub source: &'static str,
}

pub static DEMOS: &[Demo] = &[
    Demo {
        name: "calc",
        source: include_str!("../examples/calc.bas"),
    },
    Demo {
        name: "factorial",
        source: include_str!("../examples/factorial.bas"),
    },
    Demo {
        name: "count",
        source: include_str!("../examples/count.bas"),
    },
    Demo {
        name: "fibonacci",
        source: include_str!("../examples/fibonacci.bas"),
    },
    Demo {
        name: "fizzbuzz",
        source: include_str!("../examples/fizzbuzz.bas"),
    },
    Demo {
        name: "hello",
        source: include_str!("../examples/hello.bas"),
    },
    Demo {
        name: "memdump",
        source: include_str!("../examples/memdump.bas"),
    },
    Demo {
        name: "startrek",
        source: include_str!("../examples/startrek.bas"),
    },
];

pub fn default_demo_index() -> usize {
    DEMOS.iter().position(|d| d.name == "hello").unwrap_or(0)
}
