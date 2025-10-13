use std::sync::{atomic::Ordering, Arc};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use unen_app::prelude::{Runner, RunnerData, START, STEP, STOP};
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

use crate::state::WinitState;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use winit::event_loop::EventLoopProxy;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowAttributesExtWebSys;

#[derive(Default)]
pub struct WinitRunner {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<EventLoopProxy<WinitState>>,
    state: Option<WinitState>,
    data: Option<RunnerData>,
}

impl Runner for WinitRunner {
    fn run(&mut self, data: RunnerData) -> RunnerData {
        let event_loop = EventLoop::with_user_event()
            .build()
            .expect("Failed to create new EventLoop");

        // Deconstruct the data to be used
        let RunnerData {
            mut stages,
            mut state,
            term,
        } = data;

        state = stages.get(START).execute_all(state);

        // Reconstructs the data to be stored
        let data = RunnerData {
            stages,
            state,
            term,
        };

        self.data = Some(data);
        event_loop.run_app(self).expect("Failed to run EventLoop");

        self.data.take().expect("Failed to return RunnerData.")
    }
}

impl ApplicationHandler<WinitState> for WinitRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            const CANVAS_ID: &str = "canvas";
            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        let _data = match &mut self.data {
            Some(data) => data,
            None => return,
        };

        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("Failed to create window"),
        );
        let _raw_window_handle = window
            .window_handle()
            .expect("Failed to get raw window handle")
            .as_raw();
        let _raw_window_handle = window
            .display_handle()
            .expect("Failed to get raw display handle")
            .as_raw();

        #[cfg(not(target_arch = "wasm32"))]
        {
            // If we are not on web we can use pollster to await the state
            self.state = Some(pollster::block_on(WinitState::new(window)));
        }

        #[cfg(target_arch = "wasm32")]
        {
            // Run the future asynchronously and use the proxy to send the
            // results to the event loop
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(proxy.send_event(WinitState::new(window).await)).is_ok()
                });
            }
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: WinitState) {
        // This is where proxy.send_event() ends up
        #[cfg(target_arch = "wasm32")]
        {
            event.window.request_redraw();
            event.resize(
                event.window.inner_size().width,
                event.window.inner_size().height,
            );
        }
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let data = match self.data.take() {
            Some(data) => data,
            None => return,
        };

        // Deconstruct the data to be used
        let RunnerData {
            mut stages,
            mut state,
            term,
        } = data;

        match event {
            winit::event::WindowEvent::CloseRequested => {
                state = stages.get(STOP).execute_all(state);
                term.store(true, Ordering::Relaxed);
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {
                state = stages.get(STEP).execute_all(state);
            }
            _ => {}
        }

        // Reconstructs the data to be stored
        let data = RunnerData {
            stages,
            state,
            term,
        };

        self.data = Some(data);
    }
}
