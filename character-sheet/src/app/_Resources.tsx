import {
  MajorAction,
  MinorAction,
  MinorActionLight,
} from "@/components/Icons/Actions";
import { Edge, EdgeLight } from "@/components/Icons/Edge";
import { TitleMin } from "@/components/TitleMin";
import { Flex, Box } from "@radix-ui/themes";

type ResourcesProps = {
  edgeNb: number;
  minActionNb: number;
};

export const Resources = ({ edgeNb, minActionNb }: ResourcesProps) => {
  return (
    <Flex>
      <Box pr={"4"}>
        <TitleMin title={"Actions"} />
        <Box pr={"1"} display={"inline-block"}>
          <MajorAction />
        </Box>
        {Array.from({ length: 5 }).map((_, i) => (
          <Box pr={"1"} display={"inline-block"}>
            {i < minActionNb ? <MinorAction /> : <MinorActionLight />}
          </Box>
        ))}
      </Box>
      <Box>
        <TitleMin title={"Atouts"} />
        {Array.from({ length: 7 }).map((_, i) => (
          <Box pr={"1"} display={"inline-block"}>
            {i < edgeNb ? <Edge /> : <EdgeLight />}
          </Box>
        ))}
      </Box>
    </Flex>
  );
};
