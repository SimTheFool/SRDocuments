import { Flex, Box } from "@radix-ui/themes";
import { RangeScores, Weapon, getSortedNumberScoresPair } from "resources";
import { MajorAction, MinorAction } from "../Icons/Actions";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TitleMin } from "../TitleMin";
import { Card } from "../Card";
import { ShotAction as ShotActionType, RangeLabels } from "resources";
import { Ruler } from "../Ruler";

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
  return (
    <Card>
      <Flex justify={"between"}>
        <Box>
          <TitleMin title={"Tir"} subtitle={`${damage}${damage_type}`} inline />
          <Space />
          {description && <ParagraphStandard>{description}</ParagraphStandard>}
          {rangeLabels && (
            <ParagraphStandard>
              <ScoresRuler
                distanceByNb={rangeLabels}
                scoresByDistance={ranges}
              />
            </ParagraphStandard>
          )}
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

const ScoresRuler = ({
  distanceByNb,
  scoresByDistance,
}: {
  distanceByNb: RangeLabels;
  scoresByDistance: RangeScores;
}) => {
  const { numbers, scores } = getSortedNumberScoresPair(
    distanceByNb,
    scoresByDistance
  );

  const formattedScores = scores.map((score) =>
    score > 0 ? `+${score}` : score
  );

  return <Ruler items={formattedScores} placeholders={numbers} />;
};
