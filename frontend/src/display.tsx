import "./App.css";
import { useEffect, useState } from "react";
import { PlayerData, getData } from "./data";

interface UserDataDisplayProps {
  name: string;
}

const UserDataDisplay: React.FC<UserDataDisplayProps> = ({ name }) => {
  const [data, setData] = useState<PlayerData | null>(null);

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

  if (!data) return null;

  if (!data.rankIconUrl) {
    return (
      <div className="flex items-center justify-center p-4">
        <p>
          No stats to display, you provided a wrong username or you have no rank
          yet
        </p>
      </div>
    );
  }

  return (
    <div className="stats">
      <p className="mk8dx_wrapper">
        {data.rankIconUrl && (
          <img src={data.rankIconUrl} className="mk8dx_logo" alt="" />
        )}
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
