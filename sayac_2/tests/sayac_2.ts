import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TekSayac } from "../target/types/tek_sayac";


describe("tek_sayac", () => {

const provider = anchor.AnchorProvider.env();


anchor.setProvider(provider);


const program = anchor.workspace.TekSayac as Program<TekSayac>;



const sayachesabi = anchor.web3.Keypair.generate();



it("Sayaç başlatılıyor", async () => {

  await program.methods
  
  .baslat()

  .accounts({

sayac: sayachesabi.publicKey,
kullanici: provider.wallet.publicKey,
systemProgram: anchor.web3.SystemProgram.programId,
  })

  .signers([sayachesabi])

  .rpc();


  const hesap = await program.account.sayacVeri.fetch(sayachesabi.publicKey);


  console.log("Başlangıç Değeri:", hesap.sayi.toString());

});

it("Sayı artırılıyor !", async () => {

  await program.methods

  .artir()

  .accounts({

    sayac: sayachesabi.publicKey,
  })

  .rpc();


  const hesap = await program.account.sayacVeri.fetch(sayachesabi.publicKey);

  console.log("Arttırıldıktan sonraki değer:", hesap.sayi.toString());

});



});