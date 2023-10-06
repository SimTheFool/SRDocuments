import { capitalize } from "@/utils/capitalize";
import { Spell as SpellType } from "resources";
import { MajorAction, MinorAction } from "../Icons/Actions";
import { Maintained } from "../Icons/Maintained";
import { SpellDistance } from "../Icons/SpellDistance";
import { SpellNature } from "../Icons/SpellNature";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import { ActionBox } from "./ActionBox";
import { interleave } from "@/utils/interleave";
import { SpellZone } from "../Icons/SpellZone";

type SpellActionProps = {
  name: string;
  action: SpellType;
};
export const SpellAction = ({
  name,
  action: { major, minor, descriptions, maintained, nature, range, type, zone },
}: SpellActionProps) => {
  const spellSubtitle = [
    range && <SpellDistance range={range} />,
    zone && <SpellZone zone={zone} />,
    nature && <SpellNature nature={nature} />,
    maintained && <Maintained />,
  ].filter((x) => x);

  return (
    <ActionBox
      title={name}
      subtitle={interleave(spellSubtitle, <Space inline />)}
      type={type}
    >
      {{
        content: Object.entries(descriptions || {}).map(
          ([key, description]) => (
            <ParagraphStandard>
              {key === "base" ? (
                <TextReplaced>{description}</TextReplaced>
              ) : (
                <>
                  <Space />
                  <Space />
                  <TitleMin
                    title={
                      <TextReplaced>{`${capitalize(key)}: `}</TextReplaced>
                    }
                    inline
                  />
                  <Space />
                  <TextReplaced>{description}</TextReplaced>
                </>
              )}
            </ParagraphStandard>
          )
        ),
        resources: (
          <>
            {Array.from({ length: major }).map((_, i) => (
              <MajorAction key={i} />
            ))}
            {Array.from({ length: minor }).map((_, i) => (
              <MinorAction key={i} />
            ))}
          </>
        ),
      }}
    </ActionBox>
  );
};
