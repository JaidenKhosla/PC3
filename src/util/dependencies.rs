use tokio::process::Command;

use crate::configuration::get_configuration_directory;

pub struct Image<'a>
{
    title: &'a str,
    local: bool
}

const JUDGE_DOCKERFILE: Image = Image{
   title: "judge",
   local: true
};

pub const SERVER_IMAGES: [Image; 1] = 
[
    Image{title: "postgres:19beta1-alpine3.24", local: false}
];

const IMAGE_PATH: &str = "assets/images"; //Relative to configuration directory.

pub async fn verify_dependencies(is_server: bool, is_judge: bool) -> Result<bool, tokio::process::ChildStderr>
{
    let mut images: Vec<&Image> = vec![];

    if is_server
    {
        SERVER_IMAGES.iter().for_each(|i| images.push(i));
    }

    if is_judge
    {
        images.push(&JUDGE_DOCKERFILE);
    }

    for image in images.iter()
    {
        if image.local
        {   
            let title = format!("{}:latest", image.title);
            let path = format!("{}/{}.dockerfile", get_configuration_directory().join(IMAGE_PATH.to_string()).to_str().unwrap(), image.title);

            let mut command = Command::new("docker").args(["build", "-t", &title, "-f", &path, "."]).spawn().expect("build isn't working");

            command.wait().await.unwrap();
        }
        else {
            let mut command = Command::new("docker");
            command.args(["pull", "--platform", "linux/amd64", &image.title]);

            println!("{:?}", &command);


          

            command.spawn().expect("pull didn't work D:").wait().await.unwrap();

        }

    }

    Ok(true)
}

