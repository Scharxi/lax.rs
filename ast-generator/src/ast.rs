#[warn(dead_code)]
pub mod expressions {
    use std::fs::File;
    use std::io::Write;

    #[derive(Debug)]
    pub struct TreeType {
        base_name: String,
        class_name: String,
        fields: Vec<String>,
    }

    pub fn define_ast(output_dir: &str, base_name: &str, types: &[&str]) -> std::io::Result<()> {
        let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
        println!("Path: {}", path);
        let mut file = File::create(path)?;

        let mut tree_types: Vec<TreeType> = Vec::new();

        writeln!(file, "{}", "use crate::error::*;")?;
        writeln!(file, "{}", "use crate::token::*;")?;

        for t_type in types {
            let (base_class_name, args) = t_type.split_once(":").unwrap();
            let class_name = format!("{}{}", base_class_name.trim(), base_name);
            let arg_split: Vec<&str> = args.split(",").collect::<Vec<&str>>();
            let mut fields = Vec::new();
            for arg in arg_split {
                let (t2type, name) = arg.trim().split_once(" ").unwrap();
                fields.push(format!("{}: {}", name, t2type));
            }
            tree_types.push(TreeType {
                base_name: base_class_name.trim().to_owned(),
                class_name: class_name.trim().to_owned(),
                fields,
            })
        }
        writeln!(file, "\npub enum {} {{", base_name)?;
        for t in &tree_types {
            writeln!(file, "\t{}({}),", t.base_name, t.class_name)?;
        }
        writeln!(file, "}}")?;

        writeln!(file, "impl {base_name} {{")?;
        writeln!(file, "\tpub fn accept<T>(&self, {}_visitor: &dyn {base_name}Visitor<T>) -> Result<T, LaxError> {{", base_name.to_lowercase())?;
        writeln!(file, "\t\tmatch self {{")?;
        for t in &tree_types {
            writeln!(
                file,
                "\t\t\t{}::{}(v) => v.accept({}_visitor),",
                base_name,
                t.base_name,
                base_name.to_lowercase()
            )?;
        }
        writeln!(file, "\t\t}}")?;
        writeln!(file, "\t}}")?;
        writeln!(file, "}}\n")?;

        for t in &tree_types {
            writeln!(file, "\npub struct {} {{", &t.class_name)?;
            for field in &t.fields {
                writeln!(file, "\tpub {},", field)?;
            }
            writeln!(file, "}}")?;
        }

        writeln!(file, "pub trait {}Visitor<T> {{", base_name)?;

        for t in &tree_types {
            writeln!(
                file,
                "\fn visit_{}_{}(&self, expr: &{}) -> Result<T, LaxError>;",
                t.base_name.to_lowercase(),
                base_name.to_lowercase(),
                t.class_name
            )?;
        }

        writeln!(file, "}}\n")?;

        for t in &tree_types {
            writeln!(file, "impl {} {{", t.class_name)?;
            writeln!(file, "\tfn accept<T>(&self, visitor: &dyn {base_name}Visitor<T>) -> Result<T, LaxError> {{")?;
            writeln!(
                file,
                "\t\tvisitor.visit_{}_{}(self)",
                t.base_name.to_lowercase(),
                base_name.to_lowercase()
            )?;
            writeln!(file, "\t}}")?;
            writeln!(file, "}}\n")?;
        }

        Ok(())
    }
}
