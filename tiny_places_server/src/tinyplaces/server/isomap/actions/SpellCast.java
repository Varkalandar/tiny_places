package tinyplaces.server.isomap.actions;

import tinyplaces.server.data.Spell;
import tinyplaces.server.isomap.Mob;
import tinyplaces.server.isomap.Room;

/**
 * Some spells have a casting delay. This action will wait and cast them
 * at the right time.
 * 
 * @author hjm
 */
public class SpellCast implements Action 
{
    private final Mob mob;
    private final Spell spell;
    private final Mob projectile;
    private int age;
    private final int dx;
    private final int dy;
    private final int layer;    
    private boolean done;
    
    public SpellCast(Mob mob, Spell spell, Mob projectile, int layer, int dx, int dy)
    {
        this.mob = mob;
        this.spell = spell;
        this.projectile = projectile;
        this.age = 0;
        this.layer = layer;
        this.dx = dx;
        this.dy = dy;        
        this.done = false;
    }
    
    @Override
    public void process(Room room, int dt) 
    {
        age += dt;
        
        if(!done && age > spell.castTime)
        {
            projectile.x = mob.x;
            projectile.y = mob.y;
            projectile.spell = spell;

            Move move = new Move(null, projectile, layer, dx, dy, spell.speed);
            room.addAction(move);
            
            done = true;
        }
    }

    @Override
    public Mob getMob() 
    {
        return mob;
    }

    @Override
    public boolean isDone() 
    {
        return done;
    }
    
}
