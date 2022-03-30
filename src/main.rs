use cms_tools::*;
use std::io::Write;

fn main() {
    //make a client
    let client = Client::new(String::from("Gemmady"));
    let task_list = client.get_task_list(0, 8192, "", None, None).unwrap();
    let mut hm = std::collections::HashMap::<String, Vec<(f64, String)>>::new();
    for i in task_list.tasks {
        let best = client.get_stats(&i.name).unwrap().best;
        if best.is_empty() {
            let t = hm
                .entry(String::from("unsolved problems"))
                .or_insert(vec![]);
            t.push((f64::INFINITY, i.name))
        } else if best.len() == 1 || best[0].time != best[1].time {
            let t = hm.entry(best[0].username.clone()).or_insert(vec![]);
            t.push((best[0].time, i.name));
        } else {
            let t = hm.entry(String::from("tied problems")).or_insert(vec![]);
            t.push((best[0].time, i.name));
        }
    }
    let mut v: Vec<(String, Vec<(f64, String)>)> = hm.into_iter().collect();
    v.sort_by_key(|x| std::cmp::Reverse(x.1.len()));
    v.iter_mut().for_each(|i| {
        i.1.sort_by(|a, b| {
            std::cmp::Reverse(a.0)
                .partial_cmp(&std::cmp::Reverse(b.0))
                .unwrap()
        })
    });

    let content: String = v
        .iter()
        .map(|user| {
            let sublist: String = user
                .1
                .iter()
                .map(|sub| {
                    format!(
                        r#"<li>
{0:.3} <a href="https://training.olinfo.it/#/task/{1}/stats" target="_blank">{1}</a>
</li>
"#,
                        sub.0, sub.1
                    )
                })
                .collect();
            format!(
                r#"<li><details>
<summary>{} {}</summary>
<ul>
{}
</ul>
</details></li>"#,
                user.1.len(),
                user.0,
                sublist
            )
        })
        .collect();
    let html = format!(
        r#"<!doctype html>
<head lang="en">
<title>CMRanking</title>
<meta charset="UTF-8"/>
<style>
ul>li:nth-of-type(odd) {{
	background: #eeeeee;
}}
ul>li:nth-of-type(even) {{
	background: #ffffff;
}}
ul {{
	list-style-type: none;
	padding-left: 0;
}}
li {{
	padding-top: 8px;
	padding-bottom: 8px;
	padding-left: 8px;
}}
</style>
</head>
<body>
<main>
<ul>
{}
</ul>
</main>
</body>
</html>
"#,
        content
    );
    let path = std::path::Path::new("public/index.html");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut file = std::fs::File::create(path).unwrap();
    write!(file, "{}", html).ok().unwrap();
}
