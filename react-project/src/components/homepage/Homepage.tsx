import SnippetList from '../SnippetList';
import discord from './discord.svg';

export default function Homepage() {
  return (
    <div className="homepage">
      <div className="row">
        <div className="col">
          <h4>uDevGames</h4>
          <SnippetList key="links" taxonomy="udevgames" page={0} showPager={false} showHidden={false} />
        </div>
      </div>

      <div className="row">
        <div className="col">
          <h4>Links and news</h4>
          <SnippetList key="links" taxonomy="links" page={0} showPager={false} showHidden={false} />
        </div>
      </div>

      <div className="row">
        <div className="col-md">
          <h4>About</h4>
          <p>
            iDevGames is a community of game developers where we share ideas,
            knowledge, code, graphics, audio, and a helping hand to each
            other since it was founded by Carlos Camacho in 1998. Additionally
            we periodically run or participate in game jams and contests to
            encourage new game development and improve our own skills.
          </p>
        </div>
        <div className="col-md">
          <h4>Community</h4>
          <p>
            You can have a chat with the iDevGames community on our community
            <a href="https://discord.gg/r6fxVBH"
              title="Join our Discord server"
              style={{ textDecoration: "none" }}>
              <img src={discord} height="24px" alt="Discord" />
            </a>. Stop by, say hi, tell us what you're working on. We're big on
            encouragement, whether your developing apps, websites, games,
            making art, or music.
          </p>
        </div>
      </div>
    </div>
  )
}
