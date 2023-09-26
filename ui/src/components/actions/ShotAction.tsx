import { Flex, Box } from "@radix-ui/themes";
import { Weapon } from "resources";
import { MajorAction, MinorAction } from "../Icons/Actions";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TitleMin } from "../TitleMin";
import { Card } from "../Card";
import { ShotAction as ShotActionType, RangeLabels } from "resources";
import { StatTable } from "../StatTable";

/* const RangeTable = ({}: {}) => {
  return (
    <Flex wrap={"wrap"}>
      <Box px={"2"}>
        {"0"}
        <Space />
        :0
      </Box>
      <Box px={"2"}>
        {"100"}
        <Space />
        :1
      </Box>
      <Box px={"2"}>
        {"500"}
        <Space />
        :-2
      </Box>
      <Box px={"2"}>
        {"520"}
        <Space />
        :4
      </Box>
    </Flex>
  );
}; */

type ShotActionProps = {
  action: ShotActionType;
  rangeLabels?: RangeLabels;
};
export const ShotAction = ({
  action: {
    damage,
    damage_type,
    major,
    minor,
    description,
    ranges,
    ammo_consumption,
  },
  rangeLabels,
}: ShotActionProps) => {
  const rangeLabelScore = Object.entries(rangeLabels || {}).map(
    ([key, value]) => [key, ranges[value]] as const
  );

  return (
    <Card>
      <Flex justify={"between"}>
        <Box>
          <TitleMin title={"Tir"} />
          <Space />
          {!!damage && (
            <TitleMin subtitle={<>VD: {`${damage} ${damage_type}`}</>} />
          )}
          <Space />
          {description && <ParagraphStandard>{description}</ParagraphStandard>}
        </Box>
        <Box>
          {Array.from({ length: major }).map((_, i) => (
            <MajorAction key={i} />
          ))}
          {Array.from({ length: minor }).map((_, i) => (
            <MinorAction key={i} />
          ))}
        </Box>
      </Flex>
    </Card>
  );
};
