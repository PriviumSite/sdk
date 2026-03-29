import { Program, AnchorProvider } from "@coral-xyz/anchor";

export class PriviumClient {
  constructor(private program: Program, private provider: AnchorProvider) {}

  async initialize() {
    return await this.program.methods.initialize().rpc();
  }

  async shieldFunds(amount: number) {
    return await this.program.methods
      .shieldFunds(amount)
      .rpc();
  }
}
