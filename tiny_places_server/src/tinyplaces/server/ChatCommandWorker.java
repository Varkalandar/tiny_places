package tinyplaces.server;

import tinyplaces.server.isomap.Client;
import tinyplaces.server.isomap.Mob;

/**
 *
 * @author hjm
 */
public class ChatCommandWorker 
{

    void processChatCommand(CommandWorker commandWorker, Client client, String chat) 
    {
        if(chat.startsWith("/color"))
        {
            changePlayerColor(commandWorker, client, chat);
        }
    }

    private void changePlayerColor(CommandWorker commandWorker, Client client, String chat) 
    {
        Mob mob = client.mob;
        if(mob != null)
        {
            String color = chat.substring(chat.indexOf(" ")).trim();
            String [] parts = color.split(" ");
            if(parts.length < 3)
            {
                chat = "CHAT,System,1 0.5 0 1,Colors need three components at least, but only " 
                        + parts.length + " were given.";
                commandWorker.singlecast(client.getCurrentRoom().getServer(), client.socket, chat);
                return;
            }
            else if(parts.length == 3)
            {
                // add default alpha value if it was omitted
                color += " 1";
            }
            commandWorker
                    .updateMob(client, mob.id, mob.tile, mob.x, mob.y, mob.scale, color);
        }
    }    
}
