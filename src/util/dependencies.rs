use bollard::{Docker, query_parameters::CreateImageOptionsBuilder, query_parameters::CreateImageOptions};

const image_titles: [&str; 4] =
[
    "mongo", //MongoDB
    "python:3", //Python3
    "openjdk:28", //Java 28
    "gcc:12.5.0" // GCC 12.5.0 (C++)
];

pub fn verify_dependencies()
{
    let docker_engine = Docker::connect_with_defaults().unwrap();

    // let images: Vec<CreateImageOptions> = image_titles.iter().map(|image_title| ).collect::<Vec<CreateImageOptions>>();

    image_titles.iter().for_each(|image_title| {docker_engine.create_image(Some(CreateImageOptionsBuilder::default().from_image(*image_title).build()), None, None);});

}

