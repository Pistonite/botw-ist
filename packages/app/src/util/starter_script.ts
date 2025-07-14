export const STARTER_SCRIPT = `
# Click the link below to learn how to write scripts
# for the IST simulator:
#
#     https://skybook.pistonite.dev/user/index.html
#
# Here's an example script to get you started
# Navigate the script using the cursor
# And the inventory state will be shown on the right

get 1 axe 1 hammer 1 pot-lid 1 slate 1 glider 
save
get 1 diamond 2 shrooms 2 pepper 2 apple
:smug hold
  all diamond
  all but 1 shroom
  all but 1 pepper
  all but 1 apple
sell all materials
close-dialog; pick-up diamond

reload;
unequip weapon
save; reload
entangle slate; drop axe; drop
entangle glider; drop axe; drop
unequip shield
reload; save
drop all materials
reload
`;
