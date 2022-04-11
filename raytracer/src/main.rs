use regex::Regex;
use run::Run;

pub mod helper;
pub mod shape;

pub mod renderer;
pub mod raytracing;
pub mod scene;
pub mod camera;
pub mod animation;
pub mod run;

fn main()
{
    let args: Vec<String> = std::env::args().collect();

    let mut window = true;
    let mut scenes = vec![];
    let mut animation = false;
    let mut width = 0;
    let mut height = 0;
    let mut monte_carlo = None;
    let mut samples = None;

    let res_regex = Regex::new(r"^\d+x\d+$").unwrap(); // example: 800x600

    for arg in args
    {
        if arg == "cmd"
        {
            window = false;
        }
        else if arg == "animate" || arg == "animation"
        {
            animation = false;
        }
        else if arg.starts_with("monte_carlo=")
        {
            let splits: Vec<&str> = arg.split("=").collect();
            let splits_arr = splits.as_slice();

            monte_carlo = Some(splits_arr[1] == "1" || splits_arr[1] == "true");
        }
        else if arg.ends_with(".json") || arg.ends_with(".gltf") || arg.ends_with(".glb") || arg.ends_with(".obj")
        {
            scenes.push(arg);
        }
        else if res_regex.is_match(arg.as_str())
        {
            let splits: Vec<&str> = arg.split("x").collect();
            let splits_arr = splits.as_slice();

            width = splits_arr[0].parse().unwrap();
            height = splits_arr[1].parse().unwrap();
        }
        else if arg.starts_with("samples=")
        {
            let splits: Vec<&str> = arg.split("=").collect();
            let splits_arr = splits.as_slice();

            samples = Some(splits_arr[1].parse().unwrap());
        }
    }

    window = true;
    animation = true;


    //debug
    //scenes.push("scene/room.json".to_string());
    //scenes.push("scene/helmet.json".to_string());

    let mut runner = Run::new(width, height, window, scenes, animation);

    runner.init();

    //apply cmd settings
    {
        let mut rt = runner.raytracing.write().unwrap();
        if let Some(monte_carlo) = monte_carlo { rt.config.monte_carlo = monte_carlo }
        if let Some(samples) = samples { rt.config.samples = samples }
    }

    runner.start();
}