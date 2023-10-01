import {
  MajorAction,
  MinorAction,
  MinorActionLight,
} from "@/components/Icons/Actions";
import { Edge, EdgeLight } from "@/components/Icons/Edge";
import { TitleMin } from "@/components/TitleMin";
import { Flex, Box } from "@radix-ui/themes";
import { Character } from "resources";

type ResourcesProps = {
  char: Character;
};

export const Resources = ({ char }: ResourcesProps) => {
  const minActionNb = Math.max(char.stats.action_min, 5);
  const edgeNb = Math.max(char.stats.edge, char.stats.max_edge);

  return (
    <Flex>
      <Box pr={"4"}>
        <TitleMin title={"Actions"} />
        <Box pr={"2"} display={"inline-block"}></Box>
        {Array.from({ length: char.stats.action_maj }).map((_, i) => (
          <Box pr={"1"} display={"inline-block"} key={i}>
            <MajorAction />
          </Box>
        ))}
        {Array.from({ length: minActionNb }).map((_, i) => (
          <Box pr={"2"} display={"inline-block"} key={i}>
            {i < char.stats.action_min ? <MinorAction /> : <MinorActionLight />}
          </Box>
        ))}
      </Box>
      <Box>
        <TitleMin title={"Atouts"} />
        {Array.from({ length: edgeNb }).map((_, i) => (
          <Box pr={"2"} display={"inline-block"} key={i}>
            {i < char.stats.edge ? <Edge /> : <EdgeLight />}
          </Box>
        ))}
      </Box>
    </Flex>
  );
};
