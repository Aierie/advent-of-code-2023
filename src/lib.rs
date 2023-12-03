pub mod util {
    use std::fs::{read_to_string, write};

    pub fn qn<T>(callback: T)
    where
        T: FnOnce(String) -> String,
    {
        let day_and_question = std::env::current_exe()
            .unwrap()
            .to_owned()
            .file_name()
            .unwrap()
            .to_owned();
        let dir = std::env::current_dir()
            .unwrap()
            .join("src")
            .join("bin")
            .join(&day_and_question.to_string_lossy().replace(".exe", ""));

        dbg!(&dir);
        let input = read_to_string(dir.join("input")).unwrap();
        let answer = callback(input);
        write(dir.join("output"), &answer).unwrap();
        dbg!(day_and_question, answer);
    }
}
