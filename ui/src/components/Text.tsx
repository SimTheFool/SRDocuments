import { Advantage } from "./Icons/Advantage";
import { Success } from "./Icons/Success";

const ICON_ID_REGEX = /__([A-Z0-9]+)__/;

type TextProps = { children: string };
export const TextReplaced = ({ children }: TextProps) => {
  const parts = children.split(/(__[A-Z0-9]+__)/);

  const partsWithIcons = parts.map((part, i) => {
    let iconId = part.match(ICON_ID_REGEX)?.[1];
    return iconId ? (
      <Icon type={iconId as any} key={i} />
    ) : (
      part.replace(/_/g, " ")
    );
  });

  return <>{partsWithIcons}</>;
};

type IconProps = {
  type: keyof typeof iconTextsList;
};
export const Icon = ({ type }: IconProps) => {
  return iconTextsList[type];
};

const iconTextsList = {
  A1: <Advantage n={1} />,
  D1: <Advantage n={-1} />,
  SN: <Success />,
} as const;
