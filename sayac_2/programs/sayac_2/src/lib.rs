use anchor_lang::prelude::*;

declare_id!("7paSjt3bwfUFibAcNf7rbqGDTN8XawvHXho2VyzQ4m35");

#[program]
pub mod tek_sayac {


    use super::*;

pub fn baslat(ctx: Context<Baslat>) -> Result<()> {

    msg!("Sayac başlatılıyor...");

    let sayac: &mut _ = &mut ctx.accounts.sayac;


    sayac.sayi = 0;

    msg!("Sayac sıfırdan başlatıldı.");


    Ok(())
}


pub fn artir(ctx: Context<Artir>) -> Result<()> {

    msg!("Sayac artırılıyor...");

    let sayac: &mut _ = &mut ctx.accounts.sayac;

    msg!("Eski değer: {}", sayac.sayi);

    sayac.sayi += 1;

    msg!("Yeni değer: {}", sayac.sayi);

    Ok(())
}
}

#[derive(Accounts)]

pub struct Baslat<'info> {

    #[account(

        init,
        payer = kullanici,
        space = 8 + 8

    )]

    pub sayac: Account<'info, SayacVeri>,

    #[account(mut)]
    pub kullanici: Signer<'info>,

    pub system_program: Program<'info, System>,

}


#[derive(Accounts)]
pub struct Artir<'info> {

    #[account(mut)]
    pub sayac: Account<'info, SayacVeri>,

}


#[account]
pub struct SayacVeri {
    pub sayi: u64,
}

