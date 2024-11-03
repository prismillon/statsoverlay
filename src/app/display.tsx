"use client";

import Image from "next/image";
import "./App.css";
import { Suspense, useEffect, useState } from "react";
import { Data, getData } from "./data";

interface UserDataDisplayProps {
  name: string;
}

const UserDataDisplay: React.FC<UserDataDisplayProps> = ({ name }) => {
  const [data, setData] = useState<Data | null>(null);

  useEffect(() => {
    if (!name) return;

    const fetchData = async () => {
      try {
        const result = await getData(name);
        setData(result);
      } catch (error) {
        console.error("Error fetching data:", error);
      }
    };

    fetchData();
    const interval = setInterval(fetchData, 20000);

    return () => clearInterval(interval);
  }, [name]);

  if (!data) return <Suspense></Suspense>;

  if (data.rank === "") {
    return (
      <div className="flex items-center justify-center p-4">
        <p>
          No stats to display, you provided a wrong username or you have no rank
          yet
        </p>
      </div>
    );
  }

  console.log("debug: ", data);

  return (
    <div className="stats">
      <p className="mk8dx_wrapper">
        {!!data.rank && <Image src={data.rank} className="mk8dx_logo" alt="" />}
        <a>{data.mmr}</a>
        <Diff modClass={data.mod} modifier={data.diff} />
      </p>
    </div>
  );
};

function Diff({ modClass, modifier }: { modClass: string; modifier: string }) {
  return <span className={modClass}>{modifier}</span>;
}

export default UserDataDisplay;
