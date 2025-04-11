mod app;
mod login;

const CANVAS_ID: &str = "canvasId";

fn main() {
    use app::DisplayApp;
    use login::Login;
    use wasm_bindgen::JsCast as _;

    //redirect log messages to console.log
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id(CANVAS_ID)
            .expect(format!("Failed to find element with id ({CANVAS_ID})").as_str())
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect(format!("{CANVAS_ID} was not of type HtmlCanvasElement").as_str());

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(Login::new(cc)))),
            )
            .await;

        //remove loading text and spinner
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => loading_text.remove(),
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>", //TODO: add relevant crash or error details here for user
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
