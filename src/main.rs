pub mod completist;

const SAMPLE: &'static str = "
name = \"cat\"

[[argument]]
name = \"FILE\"
type = \"file+\"
optional = true

[[option]]
long = \"--show-all\"
short = \"-A\"
description = \"equivalent to -vET\"

[[option]]
long = \"--number-nonblank\"
short = \"-b\"
description = \"number nonempty output lines, overrides -n\"

[[option]]
short = \"-e\"
description = \"equivalent to -vE\"

[[option]]
long = \"--show-ends\"
short = \"-E\"
description = \"display $ at end of each line\"

[[option]]
long = \"--number\"
short = \"-n\"
description = \"number all output lines\"

[[option]]
long = \"--squeeze-blank\"
short = \"-s\"
description = \"suppress repeated empty output lines\"

[[option]]
short = \"-t\"
description = \"equivalent to -vT\"

[[option]]
long = \"--show-tabs\"
short = \"-t\"
description = \"display TAB characters as ^I\"

[[option]]
short = \"-u\"
description = \"(ignored)\"

[[option]]
long = \"--show-nonprinting\"
short = \"-v\"
description = \"use ^ and M- notation, except for LFD and TAB\"

[[option]]
long = \"--help\"
description = \"display this help and exit\"

[[option]]
long = \"--version\"
description = \"output version information and exit\"
";

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut comp = completist::Completist::new();
    comp.parse_string(SAMPLE).ok();
}
