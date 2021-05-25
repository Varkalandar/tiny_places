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
        int layer = 3;
        String color = chat.substring(chat.indexOf(" ")).trim();
        
        commandWorker
                .updateMob(client, mob.id, mob.tile, mob.x, mob.y, mob.scale, color);
    }
    
}
