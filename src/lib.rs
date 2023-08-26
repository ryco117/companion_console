// MIT License

// Copyright (c) 2023 Ryan Andersen

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE, SW_SHOW};

/// A minimal helper to manage the visibility of an allocated Windows console.
pub struct ConsoleState {
    pub handle: windows::Win32::Foundation::HWND,
    pub visible: bool,
}

impl ConsoleState {
    /// Create a new companion console window.
    /// There can be at most one console window per process.
    #[must_use]
    pub fn new(launch_visible: bool) -> Option<Self> {
        use windows::Win32::{
            System::Console::{AllocConsole, GetConsoleWindow},
            UI::WindowsAndMessaging::IsWindowVisible,
        };
        unsafe {
            // `AllocConsole` will fail if there is already a console window.
            if AllocConsole().is_ok() {
                let handle = GetConsoleWindow();
                let mut state = ConsoleState {
                    handle,
                    visible: IsWindowVisible(handle).into(),
                };

                // Ensure the desired visibility is respected.
                if launch_visible {
                    state.show();
                } else {
                    state.hide();
                }

                Some(state)
            } else {
                None
            }
        }
    }

    /// Hide the console window.
    pub fn hide(&mut self) {
        if self.visible {
            unsafe {
                ShowWindow(self.handle, SW_HIDE);
            }
            self.visible = false;
        }
    }

    /// Show the console window.
    pub fn show(&mut self) {
        if !self.visible {
            unsafe {
                ShowWindow(self.handle, SW_SHOW);
            }
            self.visible = true;
        }
    }

    /// Return the current visibility state of the console window.
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

/// Implement the `Drop` trait to ensure the console window is freed.
impl Drop for ConsoleState {
    /// Drop the allocated console window.
    fn drop(&mut self) {
        use windows::Win32::System::Console::FreeConsole;
        unsafe {
            FreeConsole().expect("Failed to free the companion console.");
        }
    }
}
