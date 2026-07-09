User: hi

GitHub Copilot: Optimized tool selectionHi.

User: configure github codespace for rust 

GitHub Copilot: I’m checking the repo layout first so I can add the smallest Codespaces/Rust setup that fits what’s already there.

Read [](file:///workspaces/Rust-for-dataengineering)

Searched for files matching `**/.devcontainer/**`, no matches

Searched for files matching `**/Cargo.toml`, no matches

Searched for files matching `**/docker-compose.yml`, no matches

Searched for files matching `**/README*`, no matches

The repo is essentially empty, so I’m treating this as a fresh Rust Codespaces scaffold rather than a tweak to an existing app. I’m checking the ignore rules, then I’ll add the devcontainer plus a minimal Cargo project so the space opens ready to build.

Read [](file:///workspaces/Rust-for-dataengineering/.gitignore)

Got last terminal command

I’ve got enough context now: there’s no Rust project yet, so the cleanest Codespaces config is a devcontainer plus a tiny starter crate. I’m applying that as a single focused change, then I’ll validate it with a build.

Apply Patch

The first patch format was off, so I’m resubmitting the same change with the required patch metadata and then I’ll run a build check.

Generating patch (31 lines) in [](file:///workspaces/Rust-for-dataengineering/.devcontainer/devcontainer.json), [](file:///workspaces/Rust-for-dataengineering/Cargo.toml), [](file:///workspaces/Rust-for-dataengineering/src/main.rs)


```

```

```

```

```

```
The scaffold is in place. I’m doing the cheapest real validation now: building the starter crate to confirm the workspace is coherent.

Ran terminal command:  cargo build

`cargo` isn’t installed in the current container, so I can’t do a real build here. I’m switching to a syntax-level check of the new config files to make sure the scaffold itself is valid.

Ran terminal command:  python3 - <<'PY'
import json
from pathlib import Path
try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib

root = Path('/workspaces/Rust-for-dataengineering')
json.loads((root / '.devcontainer/devcontainer.json').read_text())
tomllib.loads((root / 'Cargo.toml').read_text())
print('devcontainer.json and Cargo.toml parse cleanly')
PY

Configured a Rust Codespaces starter in the devcontainer, plus a minimal Cargo project in Cargo.toml and main.rs. The container uses the Rust dev image, installs `rust-analyzer`, and runs `cargo build` after creation.

I validated the new config files by parsing them successfully. I couldn’t run a full `cargo build` in this current container because `cargo` isn’t installed here; that build should work once you reopen the repo in Codespaces.

Made changes.

User: don't we need dockerfile for this project?

GitHub Copilot: Not strictly. The current devcontainer.json uses a prebuilt Rust dev image, so Codespaces already has Rust and `rust-analyzer` without needing a custom Dockerfile.

You would add a Dockerfile if you want to pin extra system packages, install a specific Rust toolchain or components, or make the environment more custom/reproducible. For a minimal starter project, the image-based setup is enough.

If you want, I can convert this to a Dockerfile-based setup next.

User: FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# Include lld linker to improve build times either by using environment variab
# RUSTFLAGS="-C link-arg =- fuse-ld=lld" or with Cargo's configuration file (i.e
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
&& apt-get -y install clang lld \
&& apt-get autoremove -y && apt-get clean -y

---------

what does it do?

GitHub Copilot: It builds a Rust dev container on top of Microsoft’s devcontainer Rust image, then installs `clang` and `lld` inside it.

What each part does:
- `FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye` starts from a prebuilt Debian Bullseye Rust environment.
- The comment explains the goal: use `lld` as the linker because it can make Rust builds faster.
- `RUN apt-get update ... apt-get -y install clang lld ...` installs:
  - `clang`: LLVM compiler tools, often useful alongside Rust toolchains
  - `lld`: LLVM’s linker, typically faster than the default linker
- `autoremove` and `clean` remove package leftovers to keep the image smaller.

So the net effect is: a Rust Codespaces image with LLVM tools and a faster linker, which can improve build times, especially for larger projects.

One small note: the tag `rust:0-1-bullseye` looks unusual. In most devcontainer examples it is usually something like `rust:1-1-bullseye`. If you want, I can turn this into a correct Dockerfile-based Codespaces setup for your repo.

User: okay, build a dockerfile

GitHub Copilot: I’m checking the current container setup so I can replace the image-only config with a Dockerfile-based one without disturbing the rest of the scaffold.

Read [](file:///workspaces/Rust-for-dataengineering/.devcontainer)

Read [](file:///workspaces/Rust-for-dataengineering/.devcontainer/devcontainer.json)

Read [](file:///workspaces/Rust-for-dataengineering/Cargo.toml)

The current setup is image-based, so the smallest useful change is to add a devcontainer Dockerfile that installs `clang` and `lld`, then point Codespaces at it. After that I’ll verify the JSON and Dockerfile syntax.

Generating patch (30 lines) in [](file:///workspaces/Rust-for-dataengineering/.devcontainer/devcontainer.json), [](file:///workspaces/Rust-for-dataengineering/.devcontainer/Dockerfile)


```

```

```

```
I’ve made the container switch; now I’m doing the best end-to-end check available here by building the Dockerfile directly.

Ran terminal command:  docker build -f .devcontainer/Dockerfile .

The Docker build produced a large log, so I’m pulling the end of it now to see whether the image actually built cleanly or stopped on a specific step.

Read [](file:///home/codespace/.vscode-remote/data/User/workspaceStorage/3fcca033/GitHub.copilot-chat/chat-session-resources/8a789b5f-c947-434b-b1d7-f7486093e851/call_gSGFQTSLvVlMCH3K0p2YvZoK__vscode-1783531435680/content.txt)

The build is still running in the background, so I’m checking the terminal once to capture the final result instead of guessing.

Checked terminal output

The build is past package installation, and I’m checking one more time for the final exit state so I can report whether the Dockerfile is good as-is.

Checked terminal output

The install step is still in flight, so I’m doing one last status pull for the actual exit code and final image result.

Checked terminal output

The build is still progressing, and I’m pulling the terminal once more for the completion line so I can finish this cleanly.

Checked terminal output

The image is still unpacking packages, so I’m giving it another moment and then checking for the final success or failure line.

Checked terminal output

It’s through the heavy package work now; I’m checking for the final Docker build completion message and exit status.

Checked terminal output

The build has reached export, which is usually the last step. I’m checking one final time for the successful completion line.

Checked terminal output

The log is at the last stage, and I’m checking whether the terminal has already exited with success so I can close this out accurately.

Checked terminal output

Made changes.

User: [Terminal 91831d6f-b31c-4df1-a56f-ec83a4798979 notification: command completed. Use send_to_terminal to send another command or kill_terminal to stop it.]
Terminal output:
cbf  1.8sProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 106.2s (4/5)                                    docker:defaultSetting up libclang-common-11-dev (1:11.0.1-2) ...       
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sSetting up libc6-dev:amd64 (2.31-13+deb11u14) ...        
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sSetting up clang-11 (1:11.0.1-2) ...                     
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sSetting up clang (1:11.0-51+nmu5) ...                    
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 106.3s (4/5)                                    docker:defaultSetting up libclang-common-11-dev (1:11.0.1-2) ...       
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sSetting up libc6-dev:amd64 (2.31-13+deb11u14) ...        
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sSetting up clang-11 (1:11.0.1-2) ...                     
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sSetting up clang (1:11.0-51+nmu5) ...                    
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 106.5s (4/5)                                    docker:defaultSetting up libclang-common-11-dev (1:11.0.1-2) ...       
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sSetting up libc6-dev:amd64 (2.31-13+deb11u14) ...        
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sSetting up clang-11 (1:11.0.1-2) ...                     
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sSetting up clang (1:11.0-51+nmu5) ...                    
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 106.6s (4/5)                                    docker:defaultSetting up libclang-common-11-dev (1:11.0.1-2) ...       
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sSetting up libc6-dev:amd64 (2.31-13+deb11u14) ...        
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sSetting up clang-11 (1:11.0.1-2) ...                     
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sSetting up clang (1:11.0-51+nmu5) ...                    
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 106.8s (4/5)                                    docker:defaultSetting up libclang-common-11-dev (1:11.0.1-2) ...       
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sSetting up libc6-dev:amd64 (2.31-13+deb11u14) ...        
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sSetting up clang-11 (1:11.0.1-2) ...                     
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sSetting up clang (1:11.0-51+nmu5) ...                    
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 106.9s (4/5)                                    docker:defaultSetting up libclang-common-11-dev (1:11.0.1-2) ...       
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sSetting up libc6-dev:amd64 (2.31-13+deb11u14) ...        
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sSetting up clang-11 (1:11.0.1-2) ...                     
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sSetting up clang (1:11.0-51+nmu5) ...                    
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 107.1s (4/5)                                    docker:defaultSetting up libclang-common-11-dev (1:11.0.1-2) ...       
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sSetting up libc6-dev:amd64 (2.31-13+deb11u14) ...        
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sSetting up clang-11 (1:11.0.1-2) ...                     
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sSetting up clang (1:11.0-51+nmu5) ...                    
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 107.1s (4/5)                                    docker:defaultSetting up libclang-common-11-dev (1:11.0.1-2) ...       
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sSetting up libc6-dev:amd64 (2.31-13+deb11u14) ...        
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sSetting up clang-11 (1:11.0.1-2) ...                     
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sSetting up clang (1:11.0-51+nmu5) ...                    
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 107.3s (4/5)                                    docker:defaultSetting up libc6-dev:amd64 (2.31-13+deb11u14) ...        
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sSetting up clang-11 (1:11.0.1-2) ...                     
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sSetting up clang (1:11.0-51+nmu5) ...                    
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sReading package lists...                                 
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 107.4s (4/5)                                    docker:defaultSetting up libc6-dev:amd64 (2.31-13+deb11u14) ...        
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sSetting up clang-11 (1:11.0.1-2) ...                     
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sSetting up clang (1:11.0-51+nmu5) ...                    
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sReading package lists...                                 
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 107.5s (4/5)                                    docker:defaultSetting up libc6-dev:amd64 (2.31-13+deb11u14) ...        
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sSetting up clang-11 (1:11.0.1-2) ...                     
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sSetting up clang (1:11.0-51+nmu5) ...                    
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sReading package lists...                                 
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 107.7s (4/5)                                    docker:defaultSetting up libc6-dev:amd64 (2.31-13+deb11u14) ...        
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sSetting up clang-11 (1:11.0.1-2) ...                     
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sSetting up clang (1:11.0-51+nmu5) ...                    
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sReading package lists...                                 
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 107.8s (4/5)                                    docker:defaultProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sReading package lists...                                 
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sBuilding dependency tree...                              
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sReading state information...                             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5s0 upgraded, 0 newly installed, 0 to remove and 76 not upg
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 108.0s (4/5)                                    docker:defaultProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sReading package lists...                                 
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sBuilding dependency tree...                              
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sReading state information...                             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5s0 upgraded, 0 newly installed, 0 to remove and 76 not upg
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 108.1s (4/5)                                    docker:defaultProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sReading package lists...                                 
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sBuilding dependency tree...                              
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sReading state information...                             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5s0 upgraded, 0 newly installed, 0 to remove and 76 not upg
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 108.3s (4/5)                                    docker:defaultProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sReading package lists...                                 
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sBuilding dependency tree...                              
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sReading state information...                             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5s0 upgraded, 0 newly installed, 0 to remove and 76 not upg
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 108.4s (4/5)                                    docker:defaultProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sReading package lists...                                 
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sBuilding dependency tree...                              
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sReading state information...                             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5s0 upgraded, 0 newly installed, 0 to remove and 76 not upg
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 108.6s (4/5)                                    docker:defaultProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sReading package lists...                                 
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sBuilding dependency tree...                              
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sReading state information...                             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5s0 upgraded, 0 newly installed, 0 to remove and 76 not upg
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 108.7s (4/5)                                    docker:defaultProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0sReading package lists...                                 
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sBuilding dependency tree...                              
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sReading state information...                             
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5s0 upgraded, 0 newly installed, 0 to remove and 76 not upg
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteract[+] Building 108.8s (5/5)                                    docker:defaultProcessing triggers for man-db (2.9.4-2) ...             
 => => extracting sha256:06772a4eff3df697497bb68b4dcdeb97fdb9338b5e7d  2.5sProcessing triggers for libc-bin (2.31-13+deb11u13) ...  
 => => extracting sha256:6fd49c17bd36c59d7bf7afe446ee52f36cad8a63936  19.0sReading package lists...                                 
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2sBuilding dependency tree...                              
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3sReading state information...                             
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s0 upgraded, 0 newly installed, 0 to remove and 76 not upg
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0[+] Building 109.0s (5/6)                                    docker:defaulttracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 109.1s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 109.3s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 109.4s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 109.6s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 109.7s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 109.9s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 110.0s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 110.2s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 110.3s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 110.5s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 110.6s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 110.8s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 110.9s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 111.1s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 111.2s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 111.4s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 111.5s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 111.7s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 111.8s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 112.0s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 112.1s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 112.3s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 112.4s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 112.6s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 112.7s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 112.9s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 113.0s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 113.2s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 113.3s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 113.5s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 113.6s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 113.8s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 113.9s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 114.1s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 114.2s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 114.4s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 114.5s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 114.7s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 114.8s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 114.9s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0sting to image                                            
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0sporting layers                                           
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b[+] Building 115.1s (5/6)                                    docker:defaulttracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08b
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9stracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8stracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5s RUN apt-get update && export DEBIAN_FRONTEND=noninteract
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteractiv  26.6sting to image                                            
 => exporting to image                                                 6.3sporting layers                                           
 => => exporting layers                                           [+] Building 115.3s (5/6)                                    docker:defaultporting manifest sha256:5c8a00256622f75d28e8cc2ac50ca2341
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sporting config sha256:e6c8acc33fd7c0877be18d8d9d137acdc76
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sporting attestation manifest sha256:b73a0cb7696e5111081e1
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sporting manifest list sha256:4b0a067ecf197e5efed0b4818941
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteractiv  26.6sming to moby-dangling@sha256:4b0a067ecf197e5efed0b4818941
 => exporting to image                                                 6.4spacking to moby-dangling@sha256:4b0a067ecf197e5efed0b4818
 => => exporting layers                                           [+] Building 115.4s (5/6)                                    docker:defaultporting manifest sha256:5c8a00256622f75d28e8cc2ac50ca2341
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sporting config sha256:e6c8acc33fd7c0877be18d8d9d137acdc76
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sporting attestation manifest sha256:b73a0cb7696e5111081e1
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sporting manifest list sha256:4b0a067ecf197e5efed0b4818941
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteractiv  26.6sming to moby-dangling@sha256:4b0a067ecf197e5efed0b4818941
 => exporting to image                                                 6.6spacking to moby-dangling@sha256:4b0a067ecf197e5efed0b4818
 => => exporting layers                                           [+] Building 115.6s (5/6)                                    docker:defaultporting manifest sha256:5c8a00256622f75d28e8cc2ac50ca2341
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sporting config sha256:e6c8acc33fd7c0877be18d8d9d137acdc76
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sporting attestation manifest sha256:b73a0cb7696e5111081e1
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sporting manifest list sha256:4b0a067ecf197e5efed0b4818941
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteractiv  26.6sming to moby-dangling@sha256:4b0a067ecf197e5efed0b4818941
 => exporting to image                                                 6.7spacking to moby-dangling@sha256:4b0a067ecf197e5efed0b4818
 => => exporting layers                                           [+] Building 115.7s (5/6)                                    docker:defaultporting manifest sha256:5c8a00256622f75d28e8cc2ac50ca2341
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sporting config sha256:e6c8acc33fd7c0877be18d8d9d137acdc76
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sporting attestation manifest sha256:b73a0cb7696e5111081e1
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sporting manifest list sha256:4b0a067ecf197e5efed0b4818941
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteractiv  26.6sming to moby-dangling@sha256:4b0a067ecf197e5efed0b4818941
 => exporting to image                                                 6.9spacking to moby-dangling@sha256:4b0a067ecf197e5efed0b4818
 => => exporting layers                                           [+] Building 115.9s (5/6)                                    docker:defaultporting manifest sha256:5c8a00256622f75d28e8cc2ac50ca2341
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sporting config sha256:e6c8acc33fd7c0877be18d8d9d137acdc76
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sporting attestation manifest sha256:b73a0cb7696e5111081e1
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sporting manifest list sha256:4b0a067ecf197e5efed0b4818941
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteractiv  26.6sming to moby-dangling@sha256:4b0a067ecf197e5efed0b4818941
 => exporting to image                                                 7.0spacking to moby-dangling@sha256:4b0a067ecf197e5efed0b4818
 => => exporting layers                                           [+] Building 116.0s (5/6)                                    docker:defaultporting manifest sha256:5c8a00256622f75d28e8cc2ac50ca2341
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sporting config sha256:e6c8acc33fd7c0877be18d8d9d137acdc76
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sporting attestation manifest sha256:b73a0cb7696e5111081e1
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sporting manifest list sha256:4b0a067ecf197e5efed0b4818941
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteractiv  26.6sming to moby-dangling@sha256:4b0a067ecf197e5efed0b4818941
 => exporting to image                                                 7.2spacking to moby-dangling@sha256:4b0a067ecf197e5efed0b4818
 => => exporting layers                                           [+] Building 116.2s (5/6)                                    docker:defaultporting manifest sha256:5c8a00256622f75d28e8cc2ac50ca2341
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sporting config sha256:e6c8acc33fd7c0877be18d8d9d137acdc76
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sporting attestation manifest sha256:b73a0cb7696e5111081e1
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sporting manifest list sha256:4b0a067ecf197e5efed0b4818941
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteractiv  26.6sming to moby-dangling@sha256:4b0a067ecf197e5efed0b4818941
 => exporting to image                                                 7.3spacking to moby-dangling@sha256:4b0a067ecf197e5efed0b4818
 => => exporting layers                                           [+] Building 116.3s (5/6)                                    docker:defaultporting manifest sha256:5c8a00256622f75d28e8cc2ac50ca2341
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sporting config sha256:e6c8acc33fd7c0877be18d8d9d137acdc76
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sporting attestation manifest sha256:b73a0cb7696e5111081e1
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sporting manifest list sha256:4b0a067ecf197e5efed0b4818941
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteractiv  26.6sming to moby-dangling@sha256:4b0a067ecf197e5efed0b4818941
 => exporting to image                                                 7.5spacking to moby-dangling@sha256:4b0a067ecf197e5efed0b4818
 => => exporting layers                                           [+] Building 116.3s (5/6)                                    docker:defaultporting manifest sha256:5c8a00256622f75d28e8cc2ac50ca2341
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9sporting config sha256:e6c8acc33fd7c0877be18d8d9d137acdc76
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8sporting attestation manifest sha256:b73a0cb7696e5111081e1
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5sporting manifest list sha256:4b0a067ecf197e5efed0b4818941
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteractiv  26.6sming to moby-dangling@sha256:4b0a067ecf197e5efed0b4818941
 => exporting to image                                                 7.5spacking to moby-dangling@sha256:4b0a067ecf197e5efed0b4818
 => => exporting layers                                           [+] Building 116.5s (6/6) FINISHED                           docker:defaultporting manifest sha256:5c8a00256622f75d28e8cc2ac50ca2341
 => [internal] load build definition from Dockerfile                   0.0sporting config sha256:e6c8acc33fd7c0877be18d8d9d137acdc76
 => => transferring dockerfile: 379B                                   0.0sporting attestation manifest sha256:b73a0cb7696e5111081e1
 => [internal] load metadata for mcr.microsoft.com/devcontainers/rust  0.6sporting manifest list sha256:4b0a067ecf197e5efed0b4818941
 => [internal] load .dockerignore                                      0.1sming to moby-dangling@sha256:4b0a067ecf197e5efed0b4818941
 => => transferring context: 2B                                        0.0spacking to moby-dangling@sha256:4b0a067ecf197e5efed0b4818
 => [1/2] FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye@sha  81.5s
 => => resolve mcr.microsoft.com/devcontainers/rust:1-1-bullseye@sha2  0.0s
 => => sha256:42d0d13f3d845eaa4c5c3fac480643752b9 155.56MB / 155.56MB  6.1s
 => => sha256:5586939a709aaf0a895755c43932dfde36c36 66.29MB / 66.29MB  6.1s
 => => sha256:49beb5d3393920bed4b74fd6812ddc8248e42 49.61MB / 49.61MB  6.1s
 => => sha256:65d159f72bb8a2aec769a3461851f023b38cb160b80 233B / 233B  0.2s
 => => sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e2 224B / 224B  0.1s
 => => sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fc 135B / 135B  0.1s
 => => sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943 408B / 408B  0.1s
 => => sha256:678459a459dcd6a035698441c64c30130c57d 14.40MB / 14.40MB  5.5s
 => => sha256:6b35768b15c47fe75ef345baf1601f0032 205.06MB / 205.06MB  10.5s
 => => sha256:6fd49c17bd36c59d7bf7afe446ee52f36c 197.14MB / 197.14MB  10.5s
 => => sha256:06772a4eff3df697497bb68b4dcdeb97fdb9 54.76MB / 54.76MB  10.5s
 => => sha256:06408a499c9b569e198473b636afa8c383e45 15.77MB / 15.77MB  0.9s
 => => sha256:2d765646d883a37610a09973a079fbba4c759 53.75MB / 53.75MB  9.6s
 => => extracting sha256:2d765646d883a37610a09973a079fbba4c7596e54d18  2.3s
 => => extracting sha256:06408a499c9b569e198473b636afa8c383e459ee6fe7  3.1s
 => => extracting sha256:06772a4eff3df697497bb68b4dcdeb97fdb9338b5e7d  2.5s
 => => extracting sha256:6fd49c17bd36c59d7bf7afe446ee52f36cad8a63936  19.0s
 => => extracting sha256:6b35768b15c47fe75ef345baf1601f003231cf7fa13  12.2s
 => => extracting sha256:678459a459dcd6a035698441c64c30130c57dbee85d2  0.3s
 => => extracting sha256:8be3692f9fda4e2e4a32b6cad9b93695e2316734943e  0.0s
 => => extracting sha256:9bed3b92df8dd88416d63f28aad4e7ddc1e61b7d0fcc  0.0s
 => => extracting sha256:96be28a192c43a2a8318f8ddf8a3d6590a4d2b578e27  0.0s
 => => extracting sha256:65d159f72bb8a2aec769a3461851f023b38cb160b804  0.0s
 => => extracting sha256:49beb5d3393920bed4b74fd6812ddc8248e42f08bae7  5.9s
 => => extracting sha256:5586939a709aaf0a895755c43932dfde36c36b5c6cbf  1.8s
 => => extracting sha256:42d0d13f3d845eaa4c5c3fac480643752b9b2996b29  17.5s
 => [2/2] RUN apt-get update && export DEBIAN_FRONTEND=noninteractiv  26.6s
 => exporting to image                                                 7.5s
 => => exporting layers                                                6.0s
 => => exporting manifest sha256:5c8a00256622f75d28e8cc2ac50ca2341229  0.0s
 => => exporting config sha256:e6c8acc33fd7c0877be18d8d9d137acdc762a8  0.0s
 => => exporting attestation manifest sha256:b73a0cb7696e5111081e1299  0.0s
 => => exporting manifest list sha256:4b0a067ecf197e5efed0b48189418f9  0.0s
 => => naming to moby-dangling@sha256:4b0a067ecf197e5efed0b48189418f9  0.0s
 => => unpacking to moby-dangling@sha256:4b0a067ecf197e5efed0b4818941  1.4s
                                                                  

GitHub Copilot: Built the Dockerfile-based Codespaces setup in the devcontainer config and the Dockerfile. It now starts from the Rust dev image and installs `clang` and `lld` for faster Rust linking.

I also validated it by running `docker build -f .devcontainer/Dockerfile .`, and the build completed successfully.

User: /compact

GitHub Copilot: Compacted conversation.

User: use git diff and tell me which is best running the run in the microsoft.mcr or via docker?

GitHub Copilot: I’m checking the exact diff first so I can compare the image-based setup against the Dockerfile-based one and give you a grounded recommendation.

Ran terminal command:  git diff -- . ':!Cargo.lock'

Ran terminal command:  git status --short

The diff is small and clear: you replaced the direct Microsoft Rust image with a Dockerfile build. I’m reading the Dockerfile itself now so I can tell you whether that tradeoff is worth it for this repo.

Read [](file:///workspaces/Rust-for-dataengineering/.devcontainer/Dockerfile)

Read [](file:///workspaces/Rust-for-dataengineering/.devcontainer/devcontainer.json)

`git diff` shows the switch is from a direct devcontainer image reference to a local Docker build. The old setup was the Microsoft Rust image in devcontainer.json, and the new setup builds from Dockerfile while still starting from `mcr.microsoft.com/devcontainers/rust:1-1-bullseye`.

For this repo, Dockerfile-based is the better choice. It gives you the Microsoft base image plus your custom Rust extras (`clang` and `lld`), so it is more reproducible and better suited to a data-engineering setup that may need more system packages later. If your goal were only the fastest possible Codespaces startup with no customization, the direct MCR image would be simpler. Here, the Dockerfile is the better long-term option.

If you want, I can also show you the exact pros/cons of the two approaches in a short table.



