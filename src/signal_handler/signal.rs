#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
#[allow(unused)]
pub enum Signal {
    /* hangup */
    SIGHUP = 1,
    /* interrupt */
    SIGINT = 2,
    /* quit */
    SIGQUIT = 3,
    /* illegal instruction (not reset when caught) */
    SIGILL = 4,
    //# if ! defined(_POSIX_SOURCE)
    /* trace trap (not reset when caught) */
    SIGTRAP = 5,
    //# endif
    /* abort() */
    SIGABRT = 6,
    //    //# if ! defined(_POSIX_SOURCE)
//    /* compatibility */
//    SIGIOT = 6,
    /* EMT instruction */
    SIGEMT = 7,
    //# endif
    /* floating point exception */
    SIGFPE = 8,
    /* kill (cannot be caught or ignored) */
    SIGKILL = 9,
    //# if ! defined(_POSIX_SOURCE)
    /* bus error */
    SIGBUS = 10,
    //# endif
    /* segmentation violation */
    SIGSEGV = 11,
    //# if ! defined(_POSIX_SOURCE)
    /* bad argument to system call */
    SIGSYS = 12,
    //# endif
    /* write on a pipe with no one to read it */
    SIGPIPE = 13,
    /* alarm clock */
    SIGALRM = 14,
    /* software termination signal from kill */
    SIGTERM = 15,
    //# if ! defined(_POSIX_SOURCE)
    /* urgent condition on IO channel */
    SIGURG = 16,
    //# endif
    /* sendable stop signal not from tty */
    SIGSTOP = 17,
    /* stop signal from tty */
    SIGTSTP = 18,
    /* continue a stopped process */
    SIGCONT = 19,
    /* to parent on child stop or exit */
    SIGCHLD = 20,
    /* to readers pgrp upon background tty read */
    SIGTTIN = 21,
    /* like TTIN for output if (tp->t_local&LTOSTOP) */
    SIGTTOU = 22,
    //# if ! defined(_POSIX_SOURCE)
    /* input/output possible signal */
    SIGIO = 23,
    /* exceeded CPU time limit */
    SIGXCPU = 24,
    /* exceeded file size limit */
    SIGXFSZ = 25,
    /* virtual time alarm */
    SIGVTALRM = 26,
    /* profiling time alarm */
    SIGPROF = 27,
    /* window size changes */
    SIGWINCH = 28,
    /* information request */
    SIGINFO = 29,
    //# endif
    /* user defined signal 1 */
    SIGUSR1 = 30,
    /* user defined signal 2 */
    SIGUSR2 = 31,
}
