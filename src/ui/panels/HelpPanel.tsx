import { Version } from "data/util";
import { Category, Description } from "ui/components";
import { Page } from "ui/surfaces";

export const HelpPanel: React.FC = () => {
	return (
		<Page title="Help">
			<Category title="What is IST? What is this app?">
				<Description>
					<span className="Highlight">Inventory Slot Transfer</span>, or IST,
					is a glitch in BOTW that desyncs the
					number of items you have in the inventory
					and number of items
					<span className="Important"> the game thinks you have </span>.
					By making the game thinks we have fewer items than we actually do, all sorts of crazy things can happen.
				</Description>
				<Description>
					Because the things that can happen are too crazy, I made this tool to help visualize what is happening
					in the inventory and in the game internally for each step in an IST setup.
				</Description>
			</Category>
			<Category title="Should I use this tool?">
				<Description>
					If you are totally new to IST, you might find this tool intimidating.
					However, it is still a good document for IST setups. You can import setups made by others
					to see the inventory in each step so you can follow along in game.
				</Description>
				<Description>
					If you are familiar with the general steps in IST setups like pick up, drop, buy, sell, break slots, etc,
					this will be a great tool to help you route and optimize your setups.
				</Description>
			</Category>
			<Category title="How do I get started?">
				<Description>
					On the left, there is the <span className="Highlight">Steps</span> panel where you can add/remove/change steps.
					Each step is a command that tells the simulator to simulate an action in the game.
					I spent a great amount of time to make the language close to a human-readable language, so you can understand without knowing the details of each command
				</Description>
				<Description>
					Clicking on a step will open to the right the current inventory state.
				</Description>
				<Description>
					If you are ready to start writing your own setups, please check out the <span className="Highlight">Commands </span>
					and the <span className="Highlight">Items </span> pages by clicking on the buttons on the top.
				</Description>
			</Category>
			<Category title="I still don't understand IST">
				<Description>
					Don't worry. IST is very complicated, which is why this tool exists. Here are some more resources/tips that can help with IST.
				</Description>
				<Description useDiv>
					<ul>
						<li>
							For help understanding how reload works: <a href="https://restite.org/reload/#">https://restite.org/reload</a> by savage13
						</li>
						<li>
							IST resources in the <a href="https://discord.com/channels/269611402854006785/1029046315528753233/1029046315528753233">
								botw speedrunning discord
							</a>
						</li>
					</ul>
				</Description>
				<Description className="Primary">
					If you still can't fully understand IST, that's completely fine. Most people just memorize the setup for specific categories that people made.
				</Description>
				<Description>
					For example, here's a <a href="https://www.youtube.com/watch?v=NZBmu9hEZY0">
						tutorial
					</a> made by Player 5 for the IST in All Dungeons.
				</Description>
			</Category>

		</Page>
	);
};
