import { ClusterManager } from 'detritus-client';

import { tokens } from '../config.json';

const manager = new ClusterManager('./bot', tokens.bot);

(async () => {
  await manager.run();
  console.log('Online');
})();
