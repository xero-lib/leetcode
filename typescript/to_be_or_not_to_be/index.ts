type ToBeOrNotToBe = {
    toBe: (val: any) => boolean;
    notToBe: (val: any) => boolean;
};

function expect(val: any): ToBeOrNotToBe {
	return {
        toBe: (val_2) => {
            if (val !== val_2) { throw new Error("Not Equal"); }
			return true;
        },
        notToBe: (val_2) => {
        	if (val === val_2) { throw new Error("Equal"); }
			return true;
        }
    }
};


console.log(expect(5).toBe(5));
console.log(expect(5).notToBe(null));
// console.log(expect(5).toBe(null)); /* would throw an error */

/**
 * expect(5).toBe(5); // true
 * expect(5).notToBe(5); // throws "Equal"
 */
