#[macro_export]
macro_rules! load_resource_helper {
        //start of macro
    ($res_list:expr) => {
	{
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

#[macro_export]
macro_rules! load_resource {
    ($fn_name:ident, $s:ident, $res_list:expr, $res_paths: expr, $res_creator:expr, $res_name:expr) => {
        fn $fn_name(&mut $s, path: &Path, path_as_string: String) -> Result<usize, Error>  {
            let index = $crate::load_resource_helper!($res_list);
            let res = Some(file_err!($res_creator.$fn_name(path))?);
            $crate::load_resource_helper!($res_list, $res_paths, $res_name, index, res, path, path_as_string)
        }
        };
    ($fn_name:ident, $s:ident, $res_list:expr, $res_paths: expr, $res_creator:expr, $res_name:expr, $extra_arg:expr) => {
	fn $fn_name(&mut $s, path: &Path, path_as_string: String) -> Result<usize, Error> {
            let index = $crate::load_resource_helper!($res_list);
            let res = Some(file_err!($res_creator.$fn_name(path, $extra_arg))?);
            $crate::load_resource_helper!($res_list, $res_paths, $res_name, index, res, path, path_as_string)
        }
        
    };
}

#[macro_export]
macro_rules! load_res_start {
    ($path:ident, $path_res: expr, $self:expr, $load:ident) => { {
	let path_as_string = $path.to_string_lossy().to_string();
        match $path_res.contains_key(&path_as_string) {
            true => $path_res[&path_as_string],
            false => $self.$load($path, path_as_string)?,
        }
    }
    };
}


#[macro_export]
macro_rules!  unload_resource{
    // s        - self
    // path_map - hashmap of string, resource id
    // res_list - list of resources
    // res      - a resource to unload
    // res_type - type of resource
    // name     - name of the resource type
    ($s:ident, $path_map:expr, $res_list:expr, $res:ident, $res_type:ty , $name:expr) => {
        /// unloads the loaded resource, so that it is no longer in memory and cannot be used for drawing
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
