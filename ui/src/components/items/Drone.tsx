import { Box, Flex } from "@radix-ui/themes";
import { Character } from "resources";
import { MasonryGrid } from "../MasonryGrid";
import { Monitor } from "../Monitor";
import { StatTable } from "../StatTable";
import { ItemCard } from "./ItemCard";
import { Slot } from "./Slot";

type Drone = Exclude<Character["drones"], undefined>[string];
type DroneProps = { item: Drone; name: string };

export const Drone = ({ item, name }: DroneProps) => {
  const stats = item.stats;
  return (
    <Box>
      <Flex>
        <ItemCard item={item} name={name}>
          <StatTable
            compact
            items={[
              ["Mani", "Acc.", "Interv.", "Vit.Max"],
              [
                `${stats.maniability_flat}/${stats.maniability_rough}`,
                stats.acceleration,
                stats.step,
                stats.max_speed,
              ],
            ]}
          />
          <StatTable
            compact
            items={[
              ["Auto.", "Res.", "Sens.", "Blin."],
              [stats.autopilot, stats.resistance, stats.sensors, stats.armor],
            ]}
          />
        </ItemCard>

        <Box
          style={{
            maxWidth: "20%",
            width: "20%",
          }}
        >
          <Monitor columns={3} hit={item.hit} alwaysCurable />
        </Box>
      </Flex>
      <MasonryGrid compact columns={2}>
        {item.slots?.map((slot) => {
          return (
            <Slot size={slot.size} concealment={slot.concealment}>
              {slot.name}
            </Slot>
          );
        })}
      </MasonryGrid>
    </Box>
  );
};
