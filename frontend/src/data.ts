export interface PlayerData {
  mmr: number;
  rank: string;
  rankIconUrl: string | null;
  diff: string;
  mod: string;
}

export async function getData(name: string): Promise<PlayerData | null> {
  try {
    const response = await fetch(
      `/api/player/details?name=${encodeURIComponent(name)}`,
    );

    if (!response.ok) {
      console.error(`Error fetching data for ${name}: ${response.status}`);
      return null;
    }

    return await response.json();
  } catch (error) {
    console.error(`Error in getData for ${name}:`, error);
    return null;
  }
}
