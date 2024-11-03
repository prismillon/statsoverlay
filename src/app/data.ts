"use server";

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
import { StaticImageData } from "next/image";

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
} as const;

export type RankType = keyof typeof rankMap;

export interface Lounge {
  mmr: number;
  rank: string;
  mmrChanges: { mmrDelta: number }[];
}

export interface Data {
  mmr: number;
  rank: StaticImageData | "";
  diff: string;
  mod: string;
}

const DEFAULT_DATA: Data = {
  mmr: 0,
  rank: "",
  diff: "",
  mod: "",
};

export async function getData(name: string): Promise<Data> {
  try {
    const response = await fetch(
      `https://www.mk8dx-lounge.com/api/player/details?name=${encodeURIComponent(
        name
      )}`,
      {
        next: { revalidate: 120 },
      }
    );

    if (!response.ok) {
      console.error(`Error fetching data for ${name}: ${response.status}`);
      return DEFAULT_DATA;
    }

    const data: Lounge = await response.json();

    if (!data.mmr || !data.mmrChanges?.length || !data.rank) {
      return DEFAULT_DATA;
    }

    const rank = data.rank.replaceAll(/ \d/g, "") as RankType;

    if (!(rank in rankMap)) {
      console.error(`Invalid rank for ${name}: ${rank}`);
      return DEFAULT_DATA;
    }

    const lastDelta = data.mmrChanges[0].mmrDelta;
    const diff = lastDelta > 0 ? `+${lastDelta}` : String(lastDelta);
    const modClass =
      lastDelta > 0
        ? "modifier green"
        : lastDelta < 0
        ? "modifier red"
        : "disabled";

    return {
      mmr: data.mmr,
      rank: rankMap[rank],
      diff,
      mod: modClass,
    };
  } catch (error) {
    console.error(`Error in getData for ${name}:`, error);
    return DEFAULT_DATA;
  }
}
