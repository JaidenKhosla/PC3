use crate::language::Language;

#[expect(unused)]
pub const PYTHON: Language<'static> = Language {
    title: "Python",
    header_command: "python",
    execution_code: "python {PATH}"
};

#[expect(unused)]
pub const JAVA: Language<'static> = Language {
    title: "Java",
    header_command: "java",
    execution_code: "javac {PATH}\njava -classpath {PARENT_PATH} {FILE_NAME}"
};

pub const DEFAULT_LANGUAGE: Language<'static> = PYTHON;