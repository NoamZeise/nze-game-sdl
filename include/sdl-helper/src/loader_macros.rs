#[macro_export]
macro_rules! load {
    // path           - path to the resource
    // res_list       - list of loaded resources
    // res_paths      - hashmap of resources and paths
    // res_creator_fn - sdl2 resource creator
    // res_name       - text name of resource type
    // font_size      - size of font for font_loader 
    ($path:ident, $res_list: expr, $res_paths:expr, $res_creator_fn:expr, $res_name: expr) => { {
	$crate::__load_shape!($path, $res_list, $res_paths, $res_name, Some(file_err!($res_creator_fn.load_texture($path))?))
    }};
    ($path:ident, $res_list: expr, $res_paths:expr, $res_creator_fn:expr, $res_name: expr, $font_size:expr) => { {
	$crate::__load_shape!($path, $res_list, $res_paths, $res_name, Some(file_err!($res_creator_fn.load_font($path, $font_size))?))
    }};
}

#[macro_export]
macro_rules! unload_resource{
    // s        - self
    // path_map - hashmap of string, resource id
    // res_list - list of resources
    // res      - a resource to unload
    // res_type - type of resource
    // name     - name of the resource type
    ($(#[$($attrss:tt)*])*, $s:ident, $path_map:expr, $res_list:expr, $res:ident, $res_type:ty , $name:expr) => {
        $(#[$($attrss)*])*
        pub fn unload(&mut $s, $res: $res_type) {
	    let mut loaded_path : Option<String> = None;
            for (k, v) in $path_map.iter() {
                if *v == $res.id {
                    loaded_path = Some(k.to_string());
                    break;
                }
            }
            let loaded_path = match loaded_path {
                Some(s) => s,
                None => {
                    println!("warning: tried to free already freed {}, id: {}", $name, $res.id);
                    return;
                },
            };
            $path_map.remove(&loaded_path);
            $res_list[$res.id] = None;
        }
    };
}

//helper for load!
#[macro_export]
macro_rules! __load_resource_helper {
    //start of macro
    ($res_list:expr) => {{
            let mut index : Option<usize> = None;
            for (i, t) in $res_list.iter().enumerate() {
                if t.is_none() {
                    index = Some(i);
                    break;
                }
            }
            index
        }
    };
    // end of macro
    ($res_list: expr, $res_paths: expr, $res_name: expr, $ind: ident, $res:ident, $path:ident, $path_as_string:ident) => {
        {
            let $ind = match $ind {
                None => {
                    $res_list.push($res);
                    $res_list.len() - 1
                },
                Some(i) => {
                    $res_list[i] = $res;
                    i
                }
            };
            $res_paths.insert($path_as_string, $ind);
            println!("loaded {} - id: {} - path: {}", $res_name, $ind, $path.to_str().unwrap());
            Ok($ind)
        }
    };
}

// helper for load!
#[macro_export]
macro_rules! __load_shape {
    ($path:ident, $res_list: expr, $res_paths:expr, $res_name: expr, $tex:expr) => { {
	let path_as_string = $path.to_string_lossy().to_string();
        match $res_paths.contains_key(&path_as_string) {
            true => $res_paths[&path_as_string],
            false => {
                let index = $crate::__load_resource_helper!($res_list);
                let res = $tex;
                $crate::__load_resource_helper!($res_list, $res_paths, $res_name, index, res, $path, path_as_string)?
            }
        }
    }};
}

