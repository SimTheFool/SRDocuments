import { Advantage } from "./Icons/Advantage";

const ICON_ID_REGEX = /__([A-Z0-9]+)__/;

type TextProps = { children: string };
export const TextWithIcons = ({ children }: TextProps) => {
  const parts = children.split(/(__[A-Z0-9]+__)/);

  const partsWithIcons = parts.map((part) => {
    let iconId = part.match(ICON_ID_REGEX)?.[1];
    return iconId ? <Icon type={iconId as any} /> : part;
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
} as const;
