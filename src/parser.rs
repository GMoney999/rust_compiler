

pub fn get_parsing_table<'a>() -> Vec<Vec<&'a str>> {
    // Initialize a 2D vector to represent the parsing table
    let mut parsing_table: Vec<Vec<&str>> = Vec::new();

    // The first vector contains all of the non-terminals and their indexes act as the column for all of the preceding vectors
    let non_terminal_labels = vec!["", "program", "var", "begin", "end.", "integer", "write", "\"value=\",", "a", "b", "c", "d", "w", "f", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "$", "=", ";", ":", ",", "(", ")", "+", "-", "*", "/"];

    // The second vector matches the production rules of the nonterminal P
    let non_terminal_p = vec!["P", "program I PA", "err", "err", "err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err"];

    // The third vector matches the production rules of teh nonterminal PA
    let non_terminal_pa = vec!["PA", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "; PB", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The fourth vector matches the production rules of the nonterminal PB
    let non_terminal_pb = vec!["PB", "err", "var DL PC", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The fifth vector matches the production rules of the nonterminal PC
    let non_terminal_pc = vec!["PC", "err", "err", "begin SL PD", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The sixth vector matches the production rules of the nonterminal PD
    let non_terminal_pd = vec!["PD", "err", "err", "err", "end.", "err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err"];

    // The seventh vector matches the production rules of the nonterminal I
    let non_terminal_i = vec!["I", "err", "err", "err", "err", "err",  "err", "err", "L IA", "L IA","L IA","L IA","L IA","L IA", "err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err","err"];

    // The eighth vector matches the production rules of the nonterminal IA
    let non_terminal_ia = vec!["IA", "err", "err", "err", "err", "err", "err", "err", "L IA", "L IA", "L IA", "L IA", "L IA", "L IA", "DG IA", "DG IA", "DG IA", "DG IA", "DG IA", "DG IA", "DG IA", "DG IA", "DG IA", "DG IA", "err", "lambda", "lambda", "err", "lambda", "err", "lambda", "lambda", "lambda", "lambda", "lambda"];

    // The ninth vector matches the production rules of the nonterminal DL
    let non_terminal_dl = vec!["DL", "err", "err", "err", "err", "err", "err", "err", "D DA", "D DA", "D DA", "D DA", "D DA", "D DA", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The tenth vector matches the production rules of the nonterminal DA
    let non_terminal_da = vec!["DA", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", ": T DB", "err", "err", "err", "err", "err", "err", "err", ];

    // The eleventh vector matches the production rules of the nonterminal DB
    let non_terminal_db = vec!["DB", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", ";", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The twelfth vector matches the production rules of the nonterminal D
    let non_terminal_d = vec!["D", "err", "err", "err", "err", "err", "err", "err", "I , DC", "I , DC", "I , DC", "I , DC", "I , DC", "I , DC", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The thirteenth vector matches the production rules of the nonterminal DC
    let non_terminal_dc = vec!["DC", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "lambda", ", D", "err", "err", "err", "err", "err", "err"];

    // The fourteenth vector matches the production rules of the nonterminal T
    let non_terminal_t = vec!["T", "err", "err", "err", "err", "integer", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The fifteenth vector matches the production rules of the nonterminal SL
    let non_terminal_sl = vec!["SL", "err", "err", "err", "err", "err", "S SLA", "err", "S SLA", "S SLA", "S SLA", "S SLA", "S SLA", "S SLA", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The sixteenth vector matches the production rules of the nonterminal SLA
    let non_terminal_sla = vec!["SLA", "err", "err", "err", "lambda", "err", "SL", "err", "SL", "SL", "SL", "SL", "SL", "SL", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The seventeenth vector matches the production rules of the nonterminal S
    let non_terminal_s = vec!["S", "err", "err", "err", "err", "err", "W", "err", "A", "A", "A", "A", "A", "A", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The eighteenth vector matches the production rules of the nonterminal W
    let non_terminal_w = vec!["W", "err", "err", "err", "err", "err", "write WA", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The nineteenth vector matches the production rules of the nonterminal WA
    let non_terminal_wa = vec!["WA", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", ") WD", "err", "err", "err", "err", "err"];

    // The twentieth vector matches the production rules of the nonterminal WB
    let non_terminal_wb = vec!["WB", "err", "err", "err", "ST I WC", "err", "ST I WC", "ST I WC", "ST I WC", "ST I WC", "ST I WC", "ST I WC", "ST I WC", "ST I WC", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The twenty-first vector matches the production rules of the nonterminal WC
    let non_terminal_wc = vec!["WC", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", ") WD", "err", "err", "err", "err"];

    // The twenty-second vector matches the production rules of the nonterminal WD
    let non_terminal_wd = vec!["WD", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", ";", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The twenty-third vector matches the production rules of the nonterminal ST
    let non_terminal_st = vec!["ST", "err", "err", "err", "err", "err", "err", "\"value=\",", "lambda", "lambda", "lambda", "lambda", "lambda", "lambda", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The twenty-fourth vector matches the production rules of the nonterminal A
    let non_terminal_a = vec!["A", "err", "err", "err", "err", "err", "err", "err", "I AB", "I AB", "I AB", "I AB", "I AB", "I AB", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The twenty-fifth vector matches the production rules of the nonterminal AB
    let non_terminal_ab = vec!["AB", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "= E AC", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The twenty-sixth vector matches the production rules of the nonterminal AC
    let non_terminal_ac = vec!["AC", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", ";", "err", "err", "err", "err", "err", "err", "err", "err"];

    // The twenty-seventh vector matches the production rules of the nonterminal E
    let non_terminal_e = vec!["E", "err", "err", "err", "err", "err", "err", "err", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA", "err", "err", "TR EA", "err", "err", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA", "TR EA"];

    // The twenty-eighth vector matches the production rules of the nonterminal EA
    let non_terminal_ea = vec!["EA", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "lambda", "err", "err", "err", "lambda", "+ TR EA", "- TR EA", "err", "err"];

    // The twenty-ninth vector matches the production rules of the nonterminal TR
    let non_terminal_tr = vec!["TR", "err", "err", "err", "err", "err", "err", "err", "F TA", "F TA", "F TA", "F TA", "F TA", "F TA", "F TA", "F TA", "F TA", "F TA", "F TA", "F TA", "F TA", "F TA", "F TA", "F TA", "err", "err", "F TA", "err", "err", "F TA", "F TA", "F TA", "F TA", "F TA", "F TA"];

    // The thirtieth vector matches the production rules of the nonterminal TA
    let non_terminal_ta = vec!["TA", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "lambda", "err", "err", "err", "lambda", "lambda", "lambda", "* F TA", "/ F TA"];

    // The thirty-first vector matches the production rules of the nonterminal F
    let non_terminal_f = vec!["F", "err", "err", "err", "err", "err", "err", "err", "I", "I", "I", "I", "I", "I", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "err", "err", "N", "err", "err", "( E )", "N", "N", "N", "N", "N"];

    // The thirty-second vector matches the production rules of the nonterminal N
    let non_terminal_n = vec!["N", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "SN DG NA", "err", "err", "err", "SN DG NA", "SN DG NA", "SN DG NA", "SN DG NA", "SN DG NA"];

    // The thirty-third vector matches the production rules of the nonterminal NA
    let non_terminal_na = vec!["NA", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "DG NA", "DG NA", "DG NA", "DG NA", "DG NA", "DG NA", "DG NA", "DG NA", "DG NA", "DG NA", "err", "err", "lambda", "err", "err", "err", "lambda", "lambda", "lambda", "lambda", "lambda"];

    // The thirty-fourth vector matches the production rules of the nonterminal SN
    let non_terminal_sn = vec!["SN", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "lambda", "lambda", "lambda", "lambda", "lambda", "lambda", "lambda", "lambda", "lambda", "lambda", "err", "err", "err", "err", "err", "err", "err", "+", "-", "err", "err"];

    // The thirty-fifth vector matches the production rules of the nonterminal DG
    let non_terminal_dg = vec!["DG", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "err","err","err","err","err","err","err","err","err","err","err"];

    // The thirty-sixth vector matches the production rules of the nonterminal L
    let non_terminal_l = vec!["L", "err", "err", "err", "err", "err", "err", "err", "a", "b", "c", "d", "w", "f", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err", "err"];

    // Push all previously created vector into the main vector in order to create the parsing table
    parsing_table.push(non_terminal_labels);
    parsing_table.push(non_terminal_p);
    parsing_table.push(non_terminal_pa);
    parsing_table.push(non_terminal_pb);
    parsing_table.push(non_terminal_pc);
    parsing_table.push(non_terminal_pd);
    parsing_table.push(non_terminal_i);
    parsing_table.push(non_terminal_ia);
    parsing_table.push(non_terminal_dl);
    parsing_table.push(non_terminal_da);
    parsing_table.push(non_terminal_db);
    parsing_table.push(non_terminal_d);
    parsing_table.push(non_terminal_dc);
    parsing_table.push(non_terminal_t);
    parsing_table.push(non_terminal_sl);
    parsing_table.push(non_terminal_sla);
    parsing_table.push(non_terminal_s);
    parsing_table.push(non_terminal_w);
    parsing_table.push(non_terminal_wa);
    parsing_table.push(non_terminal_wb);
    parsing_table.push(non_terminal_wc);
    parsing_table.push(non_terminal_wd);
    parsing_table.push(non_terminal_st);
    parsing_table.push(non_terminal_a);
    parsing_table.push(non_terminal_ab);
    parsing_table.push(non_terminal_ac);
    parsing_table.push(non_terminal_e);
    parsing_table.push(non_terminal_ea);
    parsing_table.push(non_terminal_tr);
    parsing_table.push(non_terminal_ta);
    parsing_table.push(non_terminal_f);
    parsing_table.push(non_terminal_n);
    parsing_table.push(non_terminal_na);
    parsing_table.push(non_terminal_sn);
    parsing_table.push(non_terminal_dg);
    parsing_table.push(non_terminal_l);

    parsing_table
}

