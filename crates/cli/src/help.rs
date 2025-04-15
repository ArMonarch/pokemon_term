use crate::util::write;

/// Generate short documentation. i.e `-h`
pub fn help_short() -> String {
    use crate::flags::FLAGS;

    // max column len counter, used for calculating padding between column_1 & colums_2.
    let (mut max_col1, mut max_col2): (usize, usize) = (0, 0);

    // column_1 store flag name (short , long, negated, allias) & column_2 store the flag
    // doc_short.
    let mut columns = (Vec::<String>::new(), Vec::<String>::new());

    for flag in FLAGS.iter().copied() {
        let (mut col_1, mut col_2) = (String::new(), String::new());

        // Generate first column, the flag name if present.
        if let Some(value) = flag.name_short() {
            write(
                &mut col_1,
                format!(r"-{name_short}", name_short = char::from(value)).as_ref(),
            );
            write(&mut col_1, format!(r", ").as_ref());
        }

        write(
            &mut col_1,
            format!("--{name_long}", name_long = flag.name_long()).as_ref(),
        );

        // Add flag argument? if the flag takes argument.
        if let Some(var) = flag._doc_variable() {
            write(&mut col_1, format!("={var}").as_ref());
        }

        write(&mut col_2, flag._doc_short());

        max_col1 = max_col1.max(col_1.len());
        max_col2 = max_col2.max(col_2.len());

        columns.0.push(col_1);
        columns.1.push(col_2);
    }

    // return the format the result.

    /// Const that represent padding between the two columns -> flag_name & doc_string
    const PAD: usize = 2;

    /// Const that represent template of for help short.
    const TEMPLATE_HELP_SHORT: &str = "CLI Description. todo!()

Usage: todo!()
  poke -l | --list
  poke -n | --name (pokemon_name)
  poke -h | --help
  poke -v | --version

Arguments:
  !!arguments!!
    ";

    let result = TEMPLATE_HELP_SHORT.to_string();

    let var = "!!arguments!!";
    let mut val = String::new();

    for (i, (col_1, col_2)) in columns.0.iter().zip(columns.1.iter()).enumerate() {
        if i > 0 {
            write(&mut val, "\n  ");
        }

        let pad = max_col1 - col_1.len() + PAD;

        write(&mut val, col_1);
        write(&mut val, " ".repeat(pad).as_ref());

        write(&mut val, col_2);
    }

    result.replace(var, &val)
}

/// Generate long Documentation.
pub fn help_long() -> String {
    unimplemented!("Not Implemented")
}
