use crate::error::ErrorCode;
use libc::TIOCSTI;
use nix::{sched::CloneFlags, sys::stat::Mode};
use syscallz::{Action, Cmp, Comparator, Context, Syscall};

const EPERM: u16 = 1;
const S_ISUID: u64 = Mode::S_ISUID.bits() as u64;
const S_ISGID: u64 = Mode::S_ISGID.bits() as u64;
const CLONE_NEWUSER: u64 = CloneFlags::CLONE_NEWUSER.bits() as u64;

// Unconditional syscalls deny
const SYSCALLS_REFUSED: [Syscall; 9] = [
    Syscall::keyctl,
    Syscall::add_key,
    Syscall::request_key,
    Syscall::mbind,
    Syscall::migrate_pages,
    Syscall::move_pages,
    Syscall::set_mempolicy,
    Syscall::userfaultfd,
    Syscall::perf_event_open,
];

// Conditional syscalls deny
const SYSCALL_REFUSE_IFCOMP: [(Syscall, u32, u64); 9] = [
    (Syscall::chmod, 1, S_ISUID),
    (Syscall::chmod, 1, S_ISGID),
    (Syscall::fchmod, 1, S_ISUID),
    (Syscall::fchmod, 1, S_ISGID),
    (Syscall::fchmodat, 2, S_ISUID),
    (Syscall::fchmodat, 2, S_ISGID),
    (Syscall::unshare, 0, CLONE_NEWUSER),
    (Syscall::clone, 0, CLONE_NEWUSER),
    (Syscall::ioctl, 1, TIOCSTI),
];

pub fn set_syscalls() -> Result<(), ErrorCode> {
    log::debug!("Refusing and filter unwanted syscalls");

    if let Ok(mut ctx) = Context::init_with_action(Action::Allow) {
        for sc in SYSCALLS_REFUSED {
            refuse_syscall(&mut ctx, sc)?;
        }
        for (sc, index, biteq) in SYSCALL_REFUSE_IFCOMP {
            refuse_syscall_ifcomp(&mut ctx, sc, index, biteq)?;
        }

        if let Err(_) = ctx.load() {
            return Err(ErrorCode::SocketError(0));
        }
        Ok(())
    } else {
        Err(ErrorCode::SyscallError(1))
    }
}

fn refuse_syscall(ctx: &mut Context, sc: Syscall) -> Result<(), ErrorCode> {
    match ctx.set_action_for_syscall(Action::Errno(EPERM), sc) {
        Ok(_) => Ok(()),
        Err(_) => Err(ErrorCode::SyscallError(2)),
    }
}

fn refuse_syscall_ifcomp(
    ctx: &mut Context,
    sc: Syscall,
    index: u32,
    biteq: u64,
) -> Result<(), ErrorCode> {
    match ctx.set_rule_for_syscall(
        Action::Errno(EPERM),
        sc,
        &[Comparator::new(index, Cmp::MaskedEq, biteq, Some(biteq))],
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err(ErrorCode::SocketError(3)),
    }
}
