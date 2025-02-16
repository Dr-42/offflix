use crate::interface_gui::ui_views::ImageType;

fn unpack(mut body: String) -> Option<Vec<String>> {
    let mut images = Vec::new();
    // Find ("https) and then find the next (")
    // if the next (") is found, check if it ends with .jpg
    // if it does, add it to the list of images
    // if it doesn't, continue
    while let Some(start) = body.find("[\"https") {
        let start = start + 2;
        body = body[start..].to_string();
        let end = match body.find('"') {
            Some(e) => e,
            None => break,
        };
        let img = &body[..end];
        if img.contains("encrypted") {
            continue;
        }
        if img.ends_with(".jpg") {
            images.push(img.to_string());
        }
        body = body[end..].to_string();
    }
    Some(images)
}

fn build_url(arg: String, img_type: ImageType) -> String {
    let arg = arg + " tmdb poster";
    let arg = arg.replace(' ', "+");
    match img_type {
        ImageType::Block => {
            format!(
                "https://www.google.com/search?as_st=y&as_q={}&as_epq=&as_oq=&as_eq=&imgar=s&imgcolor=&imgtype=photo&cr=&as_sitesearch=&as_filetype=jpg&tbs=&udm=2",
                arg
            )
        }
        ImageType::Banner => {
            format!(
                "https://www.google.com/search?as_st=y&as_q={}&as_epq=&as_oq=&as_eq=&imgar=w&imgcolor=&imgtype=photo&cr=&as_sitesearch=&as_filetype=jpg&tbs=&udm=2",
                arg
            )
        }
    }
}

async fn get(url: String) -> Result<String, surf::Error> {
    let mut repl = surf::get(url.clone())
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.104 Safari/537.36").await;
    let mut res = match repl {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    loop {
        if res.status() == 302 {
            let destination = res.header("Location").unwrap().as_str();
            println!("Redirecting to: {}", destination);
            repl = surf::get(destination)
                        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.104 Safari/537.36").await;
            if let Ok(mut r) = repl {
                if r.status() == 200 {
                    return r.body_string().await;
                } else {
                    res = r;
                }
            } else {
                return Err(repl.err().unwrap());
            }
        } else {
            return res.body_string().await;
        }
    }
}

pub fn download(query: &str, path_prefix: &str, img_type: ImageType) {
    let url = build_url(query.to_string(), img_type);
    let res = futures::executor::block_on(get(url));
    let imgs = match res {
        Ok(body) => unpack(body),
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    if let Some(imgs) = imgs {
        // Create the directory if it doesn't exist
        // Chop off until the last /
        let root = path_prefix.rfind('/').unwrap();
        let path = &path_prefix[..root];
        if !std::path::Path::new(path).exists() {
            println!("Creating directory: {}", path);
            std::fs::create_dir_all(path).unwrap();
        }
        let mut done: bool = false;
        for img in imgs.iter() {
            if done {
                break;
            }
            let res = futures::executor::block_on(surf::get(img.clone()));
            match res {
                Ok(mut body) => {
                    let path = format!("{}.jpg", path_prefix);
                    let content = futures::executor::block_on(body.body_bytes()).unwrap();
                    if content.is_empty() {
                        continue;
                    }
                    // Check if the image is valid
                    let img = image::load_from_memory(&content);
                    if img.is_err() {
                        continue;
                    }
                    std::fs::write(path, content).unwrap();
                    done = true;
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    } else {
        eprintln!("Error parsing images");
    }
}
