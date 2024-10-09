use bolt_lang::*;
use position::Position;

declare_id!("DpwZi8NhGz1acpfeBLS8YNQ5fesm2rNme73YwajyLDPp");

#[system]
pub mod movement {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let position = &mut ctx.accounts.position;
        position.x += 1;
        position.y += 1;
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub position: Position,
    }

}
