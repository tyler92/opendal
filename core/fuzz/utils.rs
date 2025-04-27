// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::env;

use opendal::Scheme;

/// Loads environment variables based on the fuzz target's scheme.
///
/// It detects the scheme (e.g., "fs" or "memory") from the executable name (`argv[0]`)
/// and tries to load a matching `.fs.env` or `.memory.env` file.
/// This is needed because there is no common `.env` file in OSS-Fuzz environment.
pub fn load_dotenv_file() {
    let args: Vec<String> = env::args().collect();
    let schemes = vec![Scheme::Memory, Scheme::Fs];

    for scheme in &schemes {
        if args[0].ends_with(&scheme.to_string()) {
            // if not exist, fallback to .env
            let _ = dotenvy::from_filename_override(format!(".{scheme}.env"));
            break;
        }
    }
}
