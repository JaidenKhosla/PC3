use crate::judge::language::Language;

#[expect(unused)]
pub const PYTHON: Language<'static> = Language {
    title: "Python",
    header_command: "python",
    compiliation_code: "",
    running_code: "python {PATH}",
    compilation_time: 0
};

#[expect(unused)]
pub const JAVA: Language<'static> = Language {
    title: "Java",
    header_command: "java",
    compiliation_code: "javac {PATH}",
    running_code: "java -classpath {PARENT_PATH} {FILE_NAME}",
    compilation_time: 25
};

pub const DEFAULT_LANGUAGE: Language<'static> = PYTHON;