/// Exit the program with an appropriate exit code
/// 
/// ### Arguments
/// * `code` - The exit code to use
/// 
/// ### Returns
/// This function does not return. It will terminate the program with the specified exit code.
pub fn with_code(code: Code, message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(code as i32);
}

/// Based on unix exit codes
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Code {
    Usage = 64,       /* command line usage error */
    Dataerr = 65,     /* data format error */
    NoInput = 66,     /* cannot open input */
    NoUser = 67,      /* addressee unknown */
    NoHost = 68,      /* host name unknown */
    Unavailable = 69, /* service unavailable */
    Software = 70,    /* internal software error */
    OsErr = 71,       /* system error (e.g., can't fork) */
    OsFile = 72,      /* critical OS file missing */
    CantCreat = 73,   /* can't create (user) output file */
    IoErr = 74,       /* input/output error */
    TempFail = 75,    /* temp failure; user is invited to retry */
    Protocol = 76,    /* remote error in protocol */
    NoPerm = 77,      /* permission denied */
    Config = 78,      /* configuration error */
}
