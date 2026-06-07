use crate::programming_language_service::language::Language;

pub const PYTHON: Language<'static> = Language {
    title: "Python",
    header_command: "python",
    execution_code: "python {PATH}"
};

pub const JAVA: Language<'static> = Language {
    title: "Java",
    header_command: "java",
    execution_code: "javac {PATH}\njava -classpath {PARENT_PATH} {FILE_NAME}"
};

