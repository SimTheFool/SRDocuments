import { Theme, Text } from "@radix-ui/themes";

type TextStandard = { children?: React.ReactNode };
export const TextIndice = ({ children }: TextStandard) => {
  return (
    <Theme asChild>
      <Text
        as="span"
        style={{ display: "inline", lineHeight: 1, fontSize: "8px" }}
        color="gray"
      >
        {children}
      </Text>
    </Theme>
  );
};
