use dioxus::prelude::*;

use crate::components::{install_cmd::InstallCmd, logo::Logo, prompt::Prompt};

#[component]
pub fn Header() -> Element {
	rsx! {
		header { class: "hero",
			Logo {}
			p { class: "tagline",
				"Sexy fonts for the console"
				span { class: "cursor", aria_hidden: "true", "█" }
			}
			p { class: "lede",
				"A few ASCII fonts with colors, gradients and backgrounds, rendered by one Rust core
				for your terminal, node, the browser, the browser console and any Rust apps."
			}
			pre { class: "shell",
				Prompt {}
				"cfonts \"cfonts\" -f block -c red-blue"
			}
			h2 { class: "tree-heading", "Install" }
			ul { class: "tree",
				InstallCmd { platform: "macos", cmd: "brew install cfonts" }
				InstallCmd { platform: "arch", cmd: "yay -S cfonts" }
				InstallCmd { platform: "fedora", cmd: "sudo dnf install cfonts" }
				InstallCmd { platform: "nixos", cmd: "nix-env -iA nixos.cfonts" }
				InstallCmd { platform: "macports", cmd: "sudo port install cfonts" }
				InstallCmd { platform: "rust", cmd: "cargo install cfonts" }
				InstallCmd { platform: "node", cmd: "npm install cfonts" }
			}
		}
	}
}
