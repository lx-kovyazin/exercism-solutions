abstract class Character
{
    private readonly string type;
    protected Character(string type) => this.type = type;

    public abstract int DamagePoints(Character target);

    public virtual bool Vulnerable() => false;

    public override string ToString() => $"Character is a {type}";
}

class Warrior : Character
{
    public Warrior() : base(nameof(Warrior))
    { }

    public override int DamagePoints(Character target) => target.Vulnerable() ? 10 : 6;
}

class Wizard : Character
{
    private bool spellIsPrepared = false;

    public Wizard() : base(nameof(Wizard))
    { }

    public override int DamagePoints(Character target) => spellIsPrepared ? 12 : 3;

    public void PrepareSpell() => spellIsPrepared = true;

    public override bool Vulnerable() => !spellIsPrepared;
}
