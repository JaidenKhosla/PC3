use bollard::{Docker, plugin::ErrorDetail, query_parameters::{CreateImageOptions, CreateImageOptionsBuilder, ListImagesOptionsBuilder}};
use std::option::Option::None;
use futures::stream::TryStreamExt;


const JUDGE_IMAGES: [&str; 3] =
[
    "python:3.11.15-slim-trixie", //Python3
    "openjdk:28-ea-slim", //Java 28
    "frolvlad/alpine-gxx:latest" // GCC 12.5.0 (C++)
];

const SERVER_IMAGES: [&str; 1] = 
[
    "mongo:nanoserver" //MongoDB
];


pub async fn verify_dependencies(is_server: bool) -> Result<bool, bollard::errors::Error>
{
    let docker_engine = Docker::connect_with_defaults().unwrap();

    // let images: Vec<CreateImageOptions> = image_titles.iter().map(|image_title| ).collect::<Vec<CreateImageOptions>>();
        
    let mut image_titles = JUDGE_IMAGES.iter().collect::<Vec<&&str>>();
    
    if is_server
    {
        SERVER_IMAGES.iter().for_each(|title| image_titles.push(title));
    }

    
    for image_title in image_titles.iter()
    {
        let mut image_stream = docker_engine.create_image(Some(CreateImageOptionsBuilder::default().from_image(*image_title).build()), None, None);
        
        while let Some(item) = image_stream.try_next().await?
        {
            println!("{:?}", item);
        }; 
        
        // Ok(())
    };
    
    println!("{:?}", &image_titles);    
    let images = docker_engine.list_images(Some(ListImagesOptionsBuilder::default().all(true).build())).await.unwrap();
    println!("{}", images.len());
    for image in images
    {
        println!("-> {:?}", image);
    }

    Ok(true)
}

