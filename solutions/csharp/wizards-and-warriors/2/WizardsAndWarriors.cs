abstract class Character
{
    public abstract int DamagePoints(Character target);

    public virtual bool Vulnerable() => false;

    public override string ToString() => $"Character is a {GetType().Name}";
}

class Warrior : Character
{
    public override int DamagePoints(Character target) => target.Vulnerable() ? 10 : 6;
}

class Wizard : Character
{
    private bool spellIsPrepared = false;

    public void PrepareSpell() => spellIsPrepared = true;
    
    public override int DamagePoints(Character target) => spellIsPrepared ? 12 : 3;
    public override bool Vulnerable() => !spellIsPrepared;
}
