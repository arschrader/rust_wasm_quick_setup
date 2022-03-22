# rust_wasm_quick_setup
Clone. Change a few things. Build. Go.

Ubuntu setup

1. Name your package folder. Inside folder

$'''
cargo init --lib
''' 

2.1 That will generate a src/librs folder and file, and a Cargo.toml. Inside Cargo.toml, under [dependencies]:
'''
[dependencies]
was_bindgen = "0.2"

[lib]
crate-type = ["cdylib"]
'''

2.2 lib.rs file:
'''
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct World {
    pub width: usize
}

#[wasm_bindgen]
impl World {

    pub fn new() -> World {
        World {
            width: 5
        }
    }
}

3. NPM stuff.


<package_folder>/www$
'''
npm init -y
'''

'''
npm install --save webpack webpack-cli
'''

'''
npm install --save-dev webpack-dev-server
'''

'''
npm install --save typescript ts-loader
'''

Optional:
'''
npm install --save copy-webpack-plugin
'''

4. In package.json you'll see your projects name. Change it to:  

"<name of your proejct>" : "file:../pkg"

Under "scripts": {  add:

"dev": "webpack-dev-server"

5. Now do
<package name>/www$ '''
npm install
'''

In root
'''
<pack_name>$wasm_pack build --target web
'''
<package_name>/www$ npm run dev

6. Go to your browser. 
localhost:8080

F12. Look at the console output you should see 5. 

7. Now do a ton of reading and make stuff. 
