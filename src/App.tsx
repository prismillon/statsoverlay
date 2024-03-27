import React, { useEffect } from "react";
import "./App.css";
import { animated, useSpring } from "@react-spring/web";
import GrandMaster from "./image/grandmaster.png";
import Master from "./image/master.png";
import Diamond from "./image/diamond.png";
import Ruby from "./image/ruby.png";
import Sapphire from "./image/sapphire.png";
import Platinum from "./image/platinum.png";
import Gold from "./image/gold.png";
import Silver from "./image/silver.png";
import Bronze from "./image/bronze.png";
import Iron from "./image/iron.png";

const rankMap = {
  GrandMaster,
  Master,
  Diamond,
  Ruby,
  Sapphire,
  Platinum,
  Gold,
  Silver,
  Bronze,
  Iron,
};

export default function App() {
  const searchParams = new URLSearchParams(window.location.search);
  const name = searchParams.get("name");

  const [mmr, setMmr] = React.useState(0);
  const [rankURL, setRankURL] = React.useState("");
  const [diff, setDiff] = React.useState("");
  const [modClass, setModClass] = React.useState("");
  const props = useSpring({ mmr });

  useEffect(() => {
    const fetchData = () =>
      fetch(
        `https://api.allorigins.win/raw?url=https://www.mk8dx-lounge.com/api/player/details?name=${name}`
      )
        .then((response) => response.json())
        .then((res) => {
          console.log(res);
          setMmr(res.mmr);
          const rank = res.rank.replaceAll(/ \d/g, "");
          setRankURL(rankMap[rank as keyof typeof rankMap]);
          const lastDelta = res.mmrChanges[0].mmrDelta;
          const diff = lastDelta > 0 ? `+${lastDelta}` : lastDelta;
          setDiff(diff);
          const modClass =
            lastDelta > 0
              ? "modifier green"
              : lastDelta < 0
              ? "modifier red"
              : "disabled";
          setModClass(modClass);
          console.log(mmr, rankURL, diff, modClass);
        });

    fetchData();
    const id = setInterval(fetchData, 120000);
    return () => clearInterval(id);
  }, [name, mmr, rankURL, diff, modClass]);

  if (name === null) {
    return (
      <div>
        <p>
          no stat to display, please check the url and make sure you have
          ?name=your_name in the url
        </p>
      </div>
    );
  }
  return (
    <div className="stats">
      <p className="mk8dx_wrapper">
        <img src={rankURL} className="mk8dx_logo" alt="" />
        <animated.a>{props.mmr.to((x) => x.toFixed(0))}</animated.a>
        <Diff modClass={modClass} modifier={diff} />
      </p>
    </div>
  );
}

function Diff({ modClass, modifier }: { modClass: string; modifier: string }) {
  return <span className={modClass}>{modifier}</span>;
}
